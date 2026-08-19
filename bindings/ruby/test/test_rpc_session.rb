require_relative "test_helper"
require "json"

class RpcSessionTest < Minitest::Test
  class FakeTransport
    attr_reader :written, :started, :closed

    def initialize(lines:)
      @lines = lines.dup
      @written = []
      @started = false
      @closed = false
    end

    def start
      @started = true
    end

    def close
      @closed = true
    end

    def write_line(line)
      @written << line
    end

    def read_line
      @lines.shift
    end
  end

  def test_session_delegates_start_and_close
    transport = FakeTransport.new(lines: [])
    session = CenyslovenskoBindings::RpcSession.new(transport: transport)

    session.start
    session.close

    assert transport.started
    assert transport.closed
  end

  def test_call_builds_request_and_parses_result
    transport = FakeTransport.new(lines: [JSON.generate({ jsonrpc: "2.0", id: 1, result: { version: "0.1.370" } })])
    session = CenyslovenskoBindings::RpcSession.new(transport: transport)

    result = session.call("version.get")
    payload = JSON.parse(transport.written.first)

    assert_equal({ "version" => "0.1.370" }, result)
    assert_equal 1, payload["id"]
    assert_equal "version.get", payload["method"]
  end
end
