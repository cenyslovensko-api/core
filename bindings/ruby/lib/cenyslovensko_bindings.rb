require_relative "cenyslovensko_bindings/config"
require_relative "cenyslovensko_bindings/errors"
require_relative "cenyslovensko_bindings/domain/jsonrpc"
require_relative "cenyslovensko_bindings/adapters/command_resolver"
require_relative "cenyslovensko_bindings/adapters/subprocess_transport"
require_relative "cenyslovensko_bindings/rpc_session"
require_relative "cenyslovensko_bindings/clients/version_rpc_client"
require_relative "cenyslovensko_bindings/clients/product_rpc_client"

module CenyslovenskoBindings
  CenyslovenskoVersionRpcClient = Clients::VersionRpcClient
  CenyslovenskoProductRpcClient = Clients::ProductRpcClient
end
