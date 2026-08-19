from __future__ import annotations

import json
import os
import sys
import unittest
from pathlib import Path
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from cenyslovensko_bindings import (
    CenyslovenskoProductRpcClient,
    CenyslovenskoVersionRpcClient,
    RPC_SERVER_BIN_ENV,
    RpcClientError,
    RpcProtocolError,
)
from cenyslovensko_bindings.adapters.command_resolver import resolve_rpc_server_command


FAKE_SERVER = r"""
import json
import sys

for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    request = json.loads(line)
    method = request.get("method")
    response = {"jsonrpc": "2.0", "id": request.get("id")}
    if method == "version.get":
        response["result"] = {"version": "0.1.370"}
    elif method == "force.error":
        response["error"] = {"code": -32000, "message": "forced failure"}
    elif method == "wrong.id":
        response["id"] = 999
        response["result"] = {}
    else:
        response["error"] = {"code": -32601, "message": "Method not found"}
    print(json.dumps(response), flush=True)
"""


class RpcClientTests(unittest.TestCase):
    def _command(self) -> list[str]:
        return [sys.executable, "-u", "-c", FAKE_SERVER]

    def test_get_version_success(self) -> None:
        with CenyslovenskoVersionRpcClient(command=self._command()) as client:
            version = client.get_version()
        self.assertEqual(version, "0.1.370")

    def test_protocol_error_is_raised(self) -> None:
        with CenyslovenskoVersionRpcClient(command=self._command()) as client:
            with self.assertRaises(RpcProtocolError) as error:
                client.call("force.error")
        self.assertEqual(error.exception.code, -32000)

    def test_mismatched_id_raises_client_error(self) -> None:
        with CenyslovenskoVersionRpcClient(command=self._command()) as client:
            with self.assertRaises(RpcClientError):
                client.call("wrong.id")

    def test_product_client_uses_distinct_methods(self) -> None:
        with CenyslovenskoProductRpcClient(command=self._command()) as client:
            with self.assertRaises(RpcProtocolError) as error:
                client.get_product("123")
        self.assertEqual(error.exception.code, -32601)

    def test_uses_env_binary_path_without_cargo(self) -> None:
        with patch.dict(os.environ, {RPC_SERVER_BIN_ENV: sys.executable}, clear=False):
            command = resolve_rpc_server_command(None)
        self.assertEqual(command, [sys.executable])


if __name__ == "__main__":
    unittest.main()
