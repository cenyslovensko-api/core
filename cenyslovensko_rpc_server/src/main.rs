use anyhow::Result;
use cenyslovensko_rpc_server::adapters::env_config::RpcServerConfig;
use cenyslovensko_rpc_server::adapters::stdio_rpc_server;
use cenyslovensko_rpc_server::adapters::version_api_gateway::VersionApiGateway;
use cenyslovensko_rpc_server::application::RpcApplication;

#[tokio::main]
async fn main() -> Result<()> {
    let config = RpcServerConfig::from_env()?;
    let gateway = VersionApiGateway::new(config.web_client, config.version_path);
    let app = RpcApplication::new(gateway);
    stdio_rpc_server::run(&app).await
}
