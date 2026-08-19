require_relative "test_helper"

class JsonrpcTest < Minitest::Test
  def test_build_request_without_params
    payload = CenyslovenskoBindings::Domain::Jsonrpc.build_request(request_id: 1, method: "version.get")
    assert_equal "2.0", payload[:jsonrpc]
    assert_equal 1, payload[:id]
    assert_equal "version.get", payload[:method]
    refute payload.key?(:params)
  end

  def test_parse_response_success
    result = CenyslovenskoBindings::Domain::Jsonrpc.parse_response(
      response: { "jsonrpc" => "2.0", "id" => 1, "result" => { "ok" => true } },
      expected_request_id: 1
    )
    assert_equal({ "ok" => true }, result)
  end

  def test_parse_response_wrong_id
    assert_raises(CenyslovenskoBindings::RpcClientError) do
      CenyslovenskoBindings::Domain::Jsonrpc.parse_response(
        response: { "jsonrpc" => "2.0", "id" => 2, "result" => {} },
        expected_request_id: 1
      )
    end
  end

  def test_parse_response_protocol_error
    error = assert_raises(CenyslovenskoBindings::RpcProtocolError) do
      CenyslovenskoBindings::Domain::Jsonrpc.parse_response(
        response: { "jsonrpc" => "2.0", "id" => 1, "error" => { "code" => -32000, "message" => "boom" } },
        expected_request_id: 1
      )
    end
    assert_equal(-32000, error.code)
  end
end
