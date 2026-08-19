const { buildRequest, parseResponse } = require("./domain/jsonrpc");
const { resolveRpcServerCommand } = require("./adapters/commandResolver");
const { ChildProcessRpcTransport } = require("./adapters/childProcessTransport");

class RpcSession {
  constructor(options = {}) {
    const command = resolveRpcServerCommand(options.command, {
      env: options.env,
      moduleRoot: options.moduleRoot,
    });
    this.transport =
      options.transport ||
      new ChildProcessRpcTransport(command, {
        cwd: options.cwd,
        env: options.env,
        timeoutMs: options.timeoutMs,
      });
    this.nextId = 1;
  }

  start() {
    this.transport.start();
  }

  close() {
    this.transport.close();
  }

  async call(method, params) {
    const requestId = this.nextId;
    this.nextId += 1;
    const request = buildRequest(requestId, method, params);
    this.transport.writeLine(JSON.stringify(request));
    const responseLine = await this.transport.readLine();
    const response = JSON.parse(responseLine);
    return parseResponse(response, requestId);
  }
}

module.exports = {
  RpcSession,
};
