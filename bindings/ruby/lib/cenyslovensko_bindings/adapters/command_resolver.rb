module CenyslovenskoBindings
  module Adapters
    module CommandResolver
      module_function

      def resolve_rpc_server_command(command: nil, env: ENV, module_root: File.expand_path("..", __dir__))
        return command if command

        explicit_binary = env[RPC_SERVER_BIN_ENV]
        return [explicit_binary] if explicit_binary

        extension = Gem.win_platform? ? ".exe" : ""
        bundled_binary = File.join(module_root, "bin", "#{RPC_SERVER_BIN_NAME}#{extension}")
        return [bundled_binary] if File.exist?(bundled_binary)

        return [RPC_SERVER_BIN_NAME] if command_in_path?(RPC_SERVER_BIN_NAME)

        raise RpcClientError,
              "RPC server binary not found. Set command, set #{RPC_SERVER_BIN_ENV}, " \
              "install '#{RPC_SERVER_BIN_NAME}' in PATH, or ship the binary in lib/cenyslovensko_bindings/bin."
      end

      def command_in_path?(command)
        ENV.fetch("PATH", "").split(File::PATH_SEPARATOR).any? do |path_entry|
          File.executable?(File.join(path_entry, command))
        end
      end
    end
  end
end
