# CenySlovensko API

Rust API module for retrieving CenySlovensko API vendor data. The crate follows ports-and-adapters architecture and uses
`cenyslovensko_web_client` for HTTP configuration.

> [!NOTE]
> Intended to be used by in-process or local bindings (Python, Ruby, Node.js) through JSON-RPC calls.

> [!IMPORTANT]
> This project is not affiliated with or endorsed by CenySlovensko. It is an independent implementation.

## Examples

```rust
use cenyslovensko_vendor_api::{
    adapters::http::HttpVendorSource,
    application::get_vendors_use_case::GetVendorsUseCase,
};
use cenyslovensko_web_client::WebClientConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let web_client = WebClientConfig::new("https://api.cenyslovensko.sk/").build()?;
    let source = HttpVendorSource::new(web_client, "vendor".to_string());
    let use_case = GetVendorsUseCase::new(source);

    let vendors = use_case.execute().await?;

    println!("Found {} vendors", vendors.len());
    Ok(())
}
```
