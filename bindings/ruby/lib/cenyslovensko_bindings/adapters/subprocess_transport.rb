require "open3"
require "timeout"

module CenyslovenskoBindings
  module Adapters
    class SubprocessTransport
      def initialize(command:, cwd: nil, env: nil, timeout_seconds: 15.0)
        @command = command
        @cwd = cwd
        @env = env || {}
        @timeout_seconds = timeout_seconds
        @stdin = nil
        @stdout = nil
        @wait_thread = nil
      end

      def start
        return unless @wait_thread.nil?

        @stdin, @stdout, @stderr, @wait_thread = Open3.popen3(@env, *@command, chdir: @cwd)
      end

      def close
        return if @wait_thread.nil?

        @stdin.close unless @stdin.closed?
        @stdout.close unless @stdout.closed?
        @stderr.close unless @stderr.closed?
        @wait_thread.kill if @wait_thread.alive?
        @wait_thread = nil
      end

      def write_line(line)
        ensure_started
        @stdin.puts(line)
        @stdin.flush
      end

      def read_line
        ensure_started
        Timeout.timeout(@timeout_seconds) do
          line = @stdout.gets
          raise RpcClientError, "RPC process closed stdout" if line.nil?

          line.strip
        end
      rescue Timeout::Error => e
        raise RpcClientError, "RPC response timed out: #{e.message}"
      end

      private

      def ensure_started
        start
        raise RpcClientError, "Failed to start RPC process" if @wait_thread.nil?
      end
    end
  end
end
