from __future__ import annotations

import json
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from cenyslovensko_bindings.rpc_session import RpcSession


class FakeTransport:
    def __init__(self, responses: list[str]) -> None:
        self.responses = responses
        self.started = False
        self.closed = False
        self.written: list[str] = []

    def start(self) -> None:
        self.started = True

    def close(self) -> None:
        self.closed = True

    def write_line(self, line: str) -> None:
        self.written.append(line)

    def read_line(self) -> str:
        return self.responses.pop(0)


class RpcSessionTests(unittest.TestCase):
    def test_start_and_close_delegate_to_transport(self) -> None:
        transport = FakeTransport(responses=[])
        session = RpcSession(transport=transport)

        session.start()
        session.close()

        self.assertTrue(transport.started)
        self.assertTrue(transport.closed)

    def test_call_builds_jsonrpc_request_and_parses_result(self) -> None:
        transport = FakeTransport(
            responses=[json.dumps({"jsonrpc": "2.0", "id": 1, "result": {"ok": True}})]
        )
        session = RpcSession(transport=transport)

        result = session.call("product.search", {"query": "milk"})

        self.assertEqual(result, {"ok": True})
        sent = json.loads(transport.written[0])
        self.assertEqual(sent["id"], 1)
        self.assertEqual(sent["method"], "product.search")
        self.assertEqual(sent["params"], {"query": "milk"})

    def test_call_increments_request_id(self) -> None:
        transport = FakeTransport(
            responses=[
                json.dumps({"jsonrpc": "2.0", "id": 1, "result": {"one": 1}}),
                json.dumps({"jsonrpc": "2.0", "id": 2, "result": {"two": 2}}),
            ]
        )
        session = RpcSession(transport=transport)

        session.call("a")
        session.call("b")

        first = json.loads(transport.written[0])
        second = json.loads(transport.written[1])
        self.assertEqual(first["id"], 1)
        self.assertEqual(second["id"], 2)


if __name__ == "__main__":
    unittest.main()
