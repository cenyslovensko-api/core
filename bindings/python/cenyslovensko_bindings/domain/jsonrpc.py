from __future__ import annotations

from typing import Any

from ..config import JSONRPC_VERSION
from ..errors import RpcClientError, RpcProtocolError


def build_request(request_id: int, method: str, params: Any = None) -> dict[str, Any]:
    payload: dict[str, Any] = {"jsonrpc": JSONRPC_VERSION, "id": request_id, "method": method}
    if params is not None:
        payload["params"] = params
    return payload


def parse_response(response: dict[str, Any], expected_request_id: int) -> Any:
    if response.get("id") != expected_request_id:
        raise RpcClientError(
            "Mismatched response id: "
            f"expected {expected_request_id}, got {response.get('id')}"
        )

    error = response.get("error")
    if isinstance(error, dict):
        code = int(error.get("code", -32000))
        message = str(error.get("message", "Unknown RPC error"))
        raise RpcProtocolError(code, message)

    if "result" not in response:
        raise RpcClientError("RPC response has no 'result'")
    return response["result"]
