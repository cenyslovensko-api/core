from __future__ import annotations

import os
from typing import Any, Mapping, Sequence

from ..errors import RpcClientError
from ..ports import RpcTransport
from ..rpc_session import RpcSession


class CenyslovenskoVersionRpcClient:
    def __init__(
        self,
        command: Sequence[str] | None = None,
        cwd: str | os.PathLike[str] | None = None,
        env: Mapping[str, str] | None = None,
        timeout_seconds: float = 15.0,
        transport: RpcTransport | None = None,
        session: RpcSession | None = None,
    ) -> None:
        self._session = session or RpcSession(
            command=command,
            cwd=cwd,
            env=env,
            timeout_seconds=timeout_seconds,
            transport=transport,
        )

    def __enter__(self) -> "CenyslovenskoVersionRpcClient":
        self.start()
        return self

    def __exit__(self, exc_type: object, exc: object, tb: object) -> None:
        self.close()

    def start(self) -> None:
        self._session.start()

    def close(self) -> None:
        self._session.close()

    def call(self, method: str, params: Any = None) -> Any:
        return self._session.call(method=method, params=params)

    def get_version(self) -> str:
        response = self.call("version.get")
        version = response.get("version")
        if not isinstance(version, str):
            raise RpcClientError("Missing or invalid 'version' in response")
        return version
