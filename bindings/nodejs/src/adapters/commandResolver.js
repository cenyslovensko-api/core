const fs = require("fs");
const os = require("os");
const path = require("path");
const { execFileSync } = require("child_process");
const { RPC_SERVER_BIN_ENV, RPC_SERVER_BIN_NAME } = require("../config");
const { RpcClientError } = require("../errors");

function defaultWhich(binName) {
  try {
    const output = execFileSync("which", [binName], { encoding: "utf8" }).trim();
    return output.length > 0 ? output : null;
  } catch {
    return null;
  }
}

function resolveRpcServerCommand(command, options = {}) {
  if (command) {
    return [...command];
  }

  const env = options.env || process.env;
  const existsSync = options.existsSync || fs.existsSync;
  const whichFn = options.whichFn || defaultWhich;
  const platform = options.platform || os.platform();
  const moduleRoot = options.moduleRoot || path.resolve(__dirname, "..");

  const explicitBinary = env[RPC_SERVER_BIN_ENV];
  if (explicitBinary) {
    return [explicitBinary];
  }

  const extension = platform === "win32" ? ".exe" : "";
  const bundledBinary = path.join(moduleRoot, "bin", `${RPC_SERVER_BIN_NAME}${extension}`);
  if (existsSync(bundledBinary)) {
    return [bundledBinary];
  }

  if (whichFn(RPC_SERVER_BIN_NAME)) {
    return [RPC_SERVER_BIN_NAME];
  }

  throw new RpcClientError(
    "RPC server binary not found. Set command, set " +
      `${RPC_SERVER_BIN_ENV}, install '${RPC_SERVER_BIN_NAME}' in PATH, ` +
      "or ship the binary in src/bin for package builds."
  );
}

module.exports = {
  resolveRpcServerCommand,
};
