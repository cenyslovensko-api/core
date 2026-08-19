const { JSONRPC_VERSION } = require("../config");
const { RpcClientError, RpcProtocolError } = require("../errors");

function buildRequest(requestId, method, params) {
  const payload = {
    jsonrpc: JSONRPC_VERSION,
    id: requestId,
    method,
  };
  if (params !== undefined) {
    payload.params = params;
  }
  return payload;
}

function parseResponse(response, expectedRequestId) {
  if (response.id !== expectedRequestId) {
    throw new RpcClientError(
      `Mismatched response id: expected ${expectedRequestId}, got ${response.id}`
    );
  }

  if (response.error && typeof response.error === "object") {
    const code = Number(response.error.code ?? -32000);
    const message = String(response.error.message ?? "Unknown RPC error");
    throw new RpcProtocolError(code, message);
  }

  if (!Object.prototype.hasOwnProperty.call(response, "result")) {
    throw new RpcClientError("RPC response has no 'result'");
  }
  return response.result;
}

module.exports = {
  buildRequest,
  parseResponse,
};
