module CenyslovenskoBindings
  class RpcClientError < StandardError; end

  class RpcProtocolError < RpcClientError
    attr_reader :code

    def initialize(code, message)
      @code = code
      super("RPC error #{code}: #{message}")
    end
  end
end
