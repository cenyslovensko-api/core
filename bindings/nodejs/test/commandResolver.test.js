const test = require("node:test");
const assert = require("node:assert/strict");
const { resolveRpcServerCommand } = require("../src/adapters/commandResolver");
const { RpcClientError } = require("../src/errors");
const { RPC_SERVER_BIN_ENV } = require("../src/config");

test("returns explicit command", () => {
  assert.deepEqual(resolveRpcServerCommand(["/tmp/rpc-server"]), ["/tmp/rpc-server"]);
});

test("returns env binary when set", () => {
  const env = { [RPC_SERVER_BIN_ENV]: "/opt/rpc-server" };
  const command = resolveRpcServerCommand(null, { env });
  assert.deepEqual(command, ["/opt/rpc-server"]);
});

test("returns PATH binary", () => {
  const command = resolveRpcServerCommand(null, {
    env: {},
    existsSync: () => false,
    whichFn: () => "/usr/local/bin/cenyslovensko_rpc_server",
  });
  assert.deepEqual(command, ["cenyslovensko_rpc_server"]);
});

test("fails when no binary is found", () => {
  assert.throws(
    () =>
      resolveRpcServerCommand(null, {
        env: {},
        existsSync: () => false,
        whichFn: () => null,
      }),
    RpcClientError
  );
});
