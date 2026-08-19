module CenyslovenskoBindings
  module Domain
    module Jsonrpc
      module_function

      def build_request(request_id:, method:, params: nil)
        payload = {
          jsonrpc: JSONRPC_VERSION,
          id: request_id,
          method: method,
        }
        payload[:params] = params unless params.nil?
        payload
      end

      def parse_response(response:, expected_request_id:)
        if response["id"] != expected_request_id
          raise RpcClientError, "Mismatched response id: expected #{expected_request_id}, got #{response['id']}"
        end

        if response["error"].is_a?(Hash)
          code = response["error"]["code"] || -32000
          message = response["error"]["message"] || "Unknown RPC error"
          raise RpcProtocolError.new(code.to_i, message.to_s)
        end

        unless response.key?("result")
          raise RpcClientError, "RPC response has no 'result'"
        end

        response["result"]
      end
    end
  end
end
