from __future__ import annotations

import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from cenyslovensko_bindings.domain.jsonrpc import build_request, parse_response
from cenyslovensko_bindings.errors import RpcClientError, RpcProtocolError


class JsonRpcTests(unittest.TestCase):
    def test_build_request_without_params(self) -> None:
        request = build_request(request_id=7, method="version.get")
        self.assertEqual(request["jsonrpc"], "2.0")
        self.assertEqual(request["id"], 7)
        self.assertEqual(request["method"], "version.get")
        self.assertNotIn("params", request)

    def test_build_request_with_params(self) -> None:
        request = build_request(request_id=8, method="product.get", params={"id": "123"})
        self.assertEqual(request["params"], {"id": "123"})

    def test_parse_response_success(self) -> None:
        response = {"jsonrpc": "2.0", "id": 1, "result": {"version": "0.1.370"}}
        result = parse_response(response=response, expected_request_id=1)
        self.assertEqual(result, {"version": "0.1.370"})

    def test_parse_response_rejects_mismatched_id(self) -> None:
        response = {"jsonrpc": "2.0", "id": 2, "result": {}}
        with self.assertRaises(RpcClientError):
            parse_response(response=response, expected_request_id=1)

    def test_parse_response_raises_protocol_error(self) -> None:
        response = {"jsonrpc": "2.0", "id": 1, "error": {"code": -32000, "message": "boom"}}
        with self.assertRaises(RpcProtocolError) as error:
            parse_response(response=response, expected_request_id=1)
        self.assertEqual(error.exception.code, -32000)

    def test_parse_response_requires_result(self) -> None:
        response = {"jsonrpc": "2.0", "id": 1}
        with self.assertRaises(RpcClientError):
            parse_response(response=response, expected_request_id=1)


if __name__ == "__main__":
    unittest.main()
