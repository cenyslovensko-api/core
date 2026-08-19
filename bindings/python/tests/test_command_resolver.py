from __future__ import annotations

import os
import sys
import unittest
from pathlib import Path
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from cenyslovensko_bindings.adapters.command_resolver import resolve_rpc_server_command
from cenyslovensko_bindings.config import RPC_SERVER_BIN_ENV
from cenyslovensko_bindings.errors import RpcClientError


class CommandResolverTests(unittest.TestCase):
    def test_returns_explicit_command(self) -> None:
        self.assertEqual(resolve_rpc_server_command(["/tmp/server"]), ["/tmp/server"])

    def test_returns_env_binary_when_set(self) -> None:
        with patch.dict(os.environ, {RPC_SERVER_BIN_ENV: "/opt/cenyslovensko_rpc_server"}):
            command = resolve_rpc_server_command(None)
        self.assertEqual(command, ["/opt/cenyslovensko_rpc_server"])

    def test_returns_path_binary_when_available(self) -> None:
        with patch.dict(os.environ, {}, clear=True):
            with patch(
                "cenyslovensko_bindings.adapters.command_resolver.Path.exists", return_value=False
            ):
                with patch("cenyslovensko_bindings.adapters.command_resolver.shutil.which", return_value="/usr/local/bin/cenyslovensko_rpc_server"):
                    command = resolve_rpc_server_command(None)
        self.assertEqual(command, ["cenyslovensko_rpc_server"])

    def test_raises_when_binary_not_found(self) -> None:
        with patch.dict(os.environ, {}, clear=True):
            with patch(
                "cenyslovensko_bindings.adapters.command_resolver.Path.exists", return_value=False
            ):
                with patch("cenyslovensko_bindings.adapters.command_resolver.shutil.which", return_value=None):
                    with self.assertRaises(RpcClientError):
                        resolve_rpc_server_command(None)


if __name__ == "__main__":
    unittest.main()
