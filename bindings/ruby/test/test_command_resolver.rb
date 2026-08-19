require_relative "test_helper"

class CommandResolverTest < Minitest::Test
  def test_returns_explicit_command
    command = CenyslovenskoBindings::Adapters::CommandResolver.resolve_rpc_server_command(
      command: ["/tmp/rpc-server"],
      env: {}
    )
    assert_equal ["/tmp/rpc-server"], command
  end

  def test_returns_env_binary
    command = CenyslovenskoBindings::Adapters::CommandResolver.resolve_rpc_server_command(
      env: { CenyslovenskoBindings::RPC_SERVER_BIN_ENV => "/opt/rpc-server" }
    )
    assert_equal ["/opt/rpc-server"], command
  end
end
