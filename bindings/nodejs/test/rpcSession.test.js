const test = require("node:test");
const assert = require("node:assert/strict");
const { RpcSession } = require("../src/rpcSession");

class FakeTransport {
  constructor(lines) {
    this.lines = [...lines];
    this.started = false;
    this.closed = false;
    this.written = [];
  }
  start() {
    this.started = true;
  }
  close() {
    this.closed = true;
  }
  writeLine(line) {
    this.written.push(line);
  }
  async readLine() {
    return this.lines.shift();
  }
}

test("session delegates start/close", () => {
  const transport = new FakeTransport([]);
  const session = new RpcSession({ transport });
  session.start();
  session.close();
  assert.equal(transport.started, true);
  assert.equal(transport.closed, true);
});

test("session call sends and parses request", async () => {
  const transport = new FakeTransport([
    JSON.stringify({ jsonrpc: "2.0", id: 1, result: { version: "0.1.370" } }),
  ]);
  const session = new RpcSession({ transport });
  const result = await session.call("version.get");
  assert.deepEqual(result, { version: "0.1.370" });

  const payload = JSON.parse(transport.written[0]);
  assert.equal(payload.id, 1);
  assert.equal(payload.method, "version.get");
});
