const { RpcSession } = require("../rpcSession");

class CenyslovenskoProductRpcClient {
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

  async getProduct(productId) {
    return this.call("product.get", { id: productId });
  }
}

module.exports = {
  CenyslovenskoProductRpcClient,
};
