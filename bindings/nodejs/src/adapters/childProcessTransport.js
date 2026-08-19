const { spawn } = require("child_process");
const { RpcClientError } = require("../errors");

class ChildProcessRpcTransport {
  constructor(command, options = {}) {
    this.command = [...command];
    this.cwd = options.cwd;
    this.env = options.env;
    this.timeoutMs = options.timeoutMs ?? 15000;
    this.child = null;
    this.lines = [];
    this.waiters = [];
  }

  start() {
    if (this.child) {
      return;
    }
    const [bin, ...args] = this.command;
    this.child = spawn(bin, args, {
      cwd: this.cwd,
      env: this.env,
      stdio: ["pipe", "pipe", "pipe"],
    });

    let buffer = "";
    this.child.stdout.setEncoding("utf8");
    this.child.stdout.on("data", (chunk) => {
      buffer += chunk;
      let index = buffer.indexOf("\n");
      while (index !== -1) {
        const line = buffer.slice(0, index).trim();
        buffer = buffer.slice(index + 1);
        if (line) {
          this._enqueueLine(line);
        }
        index = buffer.indexOf("\n");
      }
    });
  }

  close() {
    if (!this.child) {
      return;
    }
    if (!this.child.killed) {
      this.child.kill();
    }
    this.child = null;
  }

  writeLine(line) {
    this._ensureStarted();
    this.child.stdin.write(`${line}\n`);
  }

  readLine() {
    if (this.lines.length > 0) {
      return Promise.resolve(this.lines.shift());
    }

    return new Promise((resolve, reject) => {
      const timer = setTimeout(() => {
        this.waiters = this.waiters.filter((w) => w.resolve !== resolve);
        reject(new RpcClientError("RPC response timed out"));
      }, this.timeoutMs);
      this.waiters.push({
        resolve: (line) => {
          clearTimeout(timer);
          resolve(line);
        },
      });
    });
  }

  _ensureStarted() {
    if (!this.child) {
      this.start();
    }
    if (!this.child) {
      throw new RpcClientError("Failed to start RPC process");
    }
    if (this.child.exitCode !== null) {
      throw new RpcClientError("RPC process is not running");
    }
  }

  _enqueueLine(line) {
    const waiter = this.waiters.shift();
    if (waiter) {
      waiter.resolve(line);
      return;
    }
    this.lines.push(line);
  }
}

module.exports = {
  ChildProcessRpcTransport,
};
