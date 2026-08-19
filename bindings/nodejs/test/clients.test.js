const test = require("node:test");
const assert = require("node:assert/strict");
const { CenyslovenskoVersionRpcClient } = require("../src/clients/versionRpcClient");
const { CenyslovenskoProductRpcClient } = require("../src/clients/productRpcClient");
const { RpcClientError } = require("../src/errors");

class FakeSession {
  constructor(responses) {
    this.responses = responses || {};
    this.calls = [];
  }
  start() {}
  close() {}
  async call(method, params) {
    this.calls.push([method, params]);
    return this.responses[method] || {};
  }
}

test("version client returns version", async () => {
  const session = new FakeSession({ "version.get": { version: "0.1.370" } });
  const client = new CenyslovenskoVersionRpcClient({ session });
  const version = await client.getVersion();
  assert.equal(version, "0.1.370");
});

test("version client rejects invalid version", async () => {
  const session = new FakeSession({ "version.get": { version: 123 } });
  const client = new CenyslovenskoVersionRpcClient({ session });
  await assert.rejects(async () => client.getVersion(), RpcClientError);
});

test("product client calls product.get", async () => {
  const session = new FakeSession({ "product.get": { id: "123" } });
  const client = new CenyslovenskoProductRpcClient({ session });
  const result = await client.getProduct("123");
  assert.deepEqual(result, { id: "123" });
  assert.deepEqual(session.calls[0], ["product.get", { id: "123" }]);
});
