from __future__ import annotations

import os
import shutil
from pathlib import Path
from typing import Sequence

from ..config import RPC_SERVER_BIN_ENV, RPC_SERVER_BIN_NAME
from ..errors import RpcClientError


def resolve_rpc_server_command(command: Sequence[str] | None) -> list[str]:
    if command is not None:
        return list(command)

    explicit_binary = os.getenv(RPC_SERVER_BIN_ENV)
    if explicit_binary:
        return [explicit_binary]

    package_root = Path(__file__).resolve().parents[1]
    extension = ".exe" if os.name == "nt" else ""
    bundled_binary = package_root / "bin" / f"{RPC_SERVER_BIN_NAME}{extension}"
    if bundled_binary.exists():
        return [str(bundled_binary)]

    if shutil.which(RPC_SERVER_BIN_NAME):
        return [RPC_SERVER_BIN_NAME]

    raise RpcClientError(
        "RPC server binary not found. Set command=..., set "
        f"{RPC_SERVER_BIN_ENV}, install '{RPC_SERVER_BIN_NAME}' in PATH, "
        "or ship the binary in cenyslovensko_bindings/bin/ for wheel builds."
    )
