from __future__ import annotations

import os
import queue
import subprocess
import threading
from pathlib import Path
from typing import Mapping, Sequence

from ..errors import RpcClientError


class SubprocessRpcTransport:
    def __init__(
        self,
        command: Sequence[str],
        cwd: str | os.PathLike[str] | None = None,
        env: Mapping[str, str] | None = None,
        timeout_seconds: float = 15.0,
    ) -> None:
        self._command = list(command)
        self._cwd = Path(cwd) if cwd is not None else None
        self._env = dict(env) if env is not None else None
        self._timeout_seconds = timeout_seconds
        self._process: subprocess.Popen[str] | None = None
        self._response_queue: queue.Queue[str] = queue.Queue()
        self._reader_thread: threading.Thread | None = None

    def start(self) -> None:
        if self._process is not None:
            return
        self._process = subprocess.Popen(
            self._command,
            cwd=str(self._cwd) if self._cwd else None,
            env=self._env,
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            bufsize=1,
        )
        self._reader_thread = threading.Thread(target=self._reader_loop, daemon=True)
        self._reader_thread.start()

    def close(self) -> None:
        process = self._process
        self._process = None
        if process is None:
            return
        if process.stdin and not process.stdin.closed:
            process.stdin.close()
        try:
            process.wait(timeout=2)
        except subprocess.TimeoutExpired:
            process.terminate()
            process.wait(timeout=2)
        if process.stdout and not process.stdout.closed:
            process.stdout.close()
        if process.stderr and not process.stderr.closed:
            process.stderr.close()

    def write_line(self, line: str) -> None:
        self._ensure_started()
        assert self._process is not None
        assert self._process.stdin is not None
        self._process.stdin.write(line + "\n")
        self._process.stdin.flush()

    def read_line(self) -> str:
        try:
            return self._response_queue.get(timeout=self._timeout_seconds)
        except queue.Empty as error:
            raise RpcClientError("RPC response timed out") from error

    def _ensure_started(self) -> None:
        if self._process is None:
            self.start()
        if self._process is None:
            raise RpcClientError("Failed to start RPC process")
        if self._process.poll() is not None:
            stderr = ""
            if self._process.stderr:
                stderr = self._process.stderr.read().strip()
            raise RpcClientError(f"RPC process is not running: {stderr}")

    def _reader_loop(self) -> None:
        process = self._process
        if process is None or process.stdout is None:
            return
        for line in process.stdout:
            stripped = line.strip()
            if stripped:
                self._response_queue.put(stripped)
