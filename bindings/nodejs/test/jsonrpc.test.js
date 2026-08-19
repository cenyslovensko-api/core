const test = require("node:test");
const assert = require("node:assert/strict");
const { buildRequest, parseResponse } = require("../src/domain/jsonrpc");
const { RpcClientError, RpcProtocolError } = require("../src/errors");

test("buildRequest includes method and id", () => {
  const payload = buildRequest(3, "version.get");
  assert.equal(payload.jsonrpc, "2.0");
  assert.equal(payload.id, 3);
  assert.equal(payload.method, "version.get");
  assert.equal(payload.params, undefined);
});

test("parseResponse returns result", () => {
  const result = parseResponse({ jsonrpc: "2.0", id: 1, result: { ok: true } }, 1);
  assert.deepEqual(result, { ok: true });
});

test("parseResponse rejects wrong id", () => {
  assert.throws(
    () => parseResponse({ jsonrpc: "2.0", id: 2, result: {} }, 1),
    RpcClientError
  );
});

test("parseResponse raises protocol error", () => {
  assert.throws(
    () =>
      parseResponse(
        { jsonrpc: "2.0", id: 1, error: { code: -32000, message: "boom" } },
        1
      ),
    RpcProtocolError
  );
});
