use anyhow::Result;
use cenyslovensko_rpc_server::adapters::env_config::RpcServerConfig;
use cenyslovensko_rpc_server::adapters::product_prices_api_gateway::ProductPricesApiGateway;
use cenyslovensko_rpc_server::adapters::stdio_rpc_server;
use cenyslovensko_rpc_server::adapters::vendor_gateway::VendorApiGateway;
use cenyslovensko_rpc_server::adapters::version_api_gateway::VersionApiGateway;
use cenyslovensko_rpc_server::application::RpcApplicationBuilder;

#[tokio::main]
async fn main() -> Result<()> {
    let config = RpcServerConfig::from_env()?;
    let app = RpcApplicationBuilder::new()
        .version_gateway(VersionApiGateway::new(
            config.web_client.to_owned(),
            config.version_path,
        ))
        .vendor_gateway(VendorApiGateway::new(
            config.web_client.to_owned(),
            config.vendor_path,
        ))
        .product_prices_gateway(ProductPricesApiGateway::new(
            config.web_client.to_owned(),
            config.product_prices_current_day_path,
        ))
        .build();
    stdio_rpc_server::run(&app).await
}
