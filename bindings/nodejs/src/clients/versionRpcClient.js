const { RpcSession } = require("../rpcSession");
const { RpcClientError } = require("../errors");

class CenyslovenskoVersionRpcClient {
  constructor(options = {}) {
    this.session = options.session || new RpcSession(options);
  }

  start() {
    this.session.start();
  }

  close() {
    this.session.close();
  }

  async call(method, params) {
    return this.session.call(method, params);
  }

  async getVersion() {
    const response = await this.call("version.get");
    if (typeof response.version !== "string") {
      throw new RpcClientError("Missing or invalid 'version' in response");
    }
    return response.version;
  }
}

module.exports = {
  CenyslovenskoVersionRpcClient,
};
