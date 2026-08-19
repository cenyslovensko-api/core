class RpcClientError extends Error {}

class RpcProtocolError extends RpcClientError {
  constructor(code, message) {
    super(`RPC error ${code}: ${message}`);
    this.code = code;
    this.message = message;
  }
}

module.exports = {
  RpcClientError,
  RpcProtocolError,
};
