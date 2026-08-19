module CenyslovenskoBindings
  module Clients
    class ProductRpcClient
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

      def get_product(product_id)
        call("product.get", { id: product_id })
      end
    end
  end
end
