require_relative "test_helper"

class ClientsTest < Minitest::Test
  class FakeSession
    attr_reader :calls

    def initialize(responses:)
      @responses = responses
      @calls = []
    end

    def start; end

    def close; end

    def call(method, params = nil)
      @calls << [method, params]
      @responses.fetch(method, {})
    end
  end

  def test_version_client_returns_version
    session = FakeSession.new(responses: { "version.get" => { "version" => "0.1.370" } })
    client = CenyslovenskoBindings::Clients::VersionRpcClient.new(session: session)
    assert_equal "0.1.370", client.get_version
  end

  def test_version_client_rejects_invalid_shape
    session = FakeSession.new(responses: { "version.get" => { "version" => 123 } })
    client = CenyslovenskoBindings::Clients::VersionRpcClient.new(session: session)
    assert_raises(CenyslovenskoBindings::RpcClientError) { client.get_version }
  end

  def test_product_client_calls_product_method
    session = FakeSession.new(responses: { "product.get" => { "id" => "123" } })
    client = CenyslovenskoBindings::Clients::ProductRpcClient.new(session: session)
    result = client.get_product("123")
    assert_equal({ "id" => "123" }, result)
    assert_equal ["product.get", { id: "123" }], session.calls.first
  end
end
