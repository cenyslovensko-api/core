class RpcClientError(Exception):
    pass


class RpcProtocolError(RpcClientError):
    def __init__(self, code: int, message: str) -> None:
        self.code = code
        self.message = message
        super().__init__(f"RPC error {code}: {message}")
