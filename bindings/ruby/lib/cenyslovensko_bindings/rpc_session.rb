require "json"

module CenyslovenskoBindings
  class RpcSession
    def initialize(command: nil, cwd: nil, env: nil, timeout_seconds: 15.0, transport: nil)
      @transport = transport || begin
        resolved_command = Adapters::CommandResolver.resolve_rpc_server_command(
          command: command,
          env: env || ENV
        )
        Adapters::SubprocessTransport.new(
          command: resolved_command,
          cwd: cwd,
          env: env,
          timeout_seconds: timeout_seconds
        )
      end
      @next_id = 1
    end

    def start
      @transport.start
    end

    def close
      @transport.close
    end

    def call(method, params = nil)
      request_id = @next_id
      @next_id += 1

      request = Domain::Jsonrpc.build_request(request_id: request_id, method: method, params: params)
      @transport.write_line(JSON.generate(request))
      response_line = @transport.read_line
      response = JSON.parse(response_line)
      Domain::Jsonrpc.parse_response(response: response, expected_request_id: request_id)
    end
  end
end
