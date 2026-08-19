from __future__ import annotations

import json
import os
import threading
from typing import Any, Mapping, Sequence

from .adapters import SubprocessRpcTransport, resolve_rpc_server_command
from .domain import build_request, parse_response
from .ports import RpcTransport


class RpcSession:
    def __init__(
        self,
        command: Sequence[str] | None = None,
        cwd: str | os.PathLike[str] | None = None,
        env: Mapping[str, str] | None = None,
        timeout_seconds: float = 15.0,
        transport: RpcTransport | None = None,
    ) -> None:
        if transport is not None:
            self._transport = transport
        else:
            resolved_command = resolve_rpc_server_command(command)
            self._transport = SubprocessRpcTransport(
                command=resolved_command,
                cwd=cwd,
                env=env,
                timeout_seconds=timeout_seconds,
            )
        self._lock = threading.Lock()
        self._next_id = 1

    def start(self) -> None:
        self._transport.start()

    def close(self) -> None:
        self._transport.close()

    def call(self, method: str, params: Any = None) -> Any:
        with self._lock:
            request_id = self._next_id
            self._next_id += 1
            request = build_request(request_id=request_id, method=method, params=params)
            self._transport.write_line(json.dumps(request))
            response_line = self._transport.read_line()
            response = json.loads(response_line)
            return parse_response(response=response, expected_request_id=request_id)
