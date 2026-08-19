module CenyslovenskoBindings
  module Clients
    class VersionRpcClient
      def initialize(session: nil, **session_options)
        @session = session || RpcSession.new(**session_options)
      end

      def start
        @session.start
      end

      def close
        @session.close
      end

      def call(method, params = nil)
        @session.call(method, params)
      end

      def get_version
        response = call("version.get")
        version = response["version"]
        raise RpcClientError, "Missing or invalid 'version' in response" unless version.is_a?(String)

        version
      end
    end
  end
end
