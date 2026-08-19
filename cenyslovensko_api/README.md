# CenySlovensko API

Umbrella crate for the CenySlovensko Rust API. Enable only the sub-crates you need, or use the `full` feature to
re-export the version and vendor APIs together with the shared `cenyslovensko_web_client` configuration layer.

> [!NOTE]
> Intended to be used by in-process or local bindings (Python, Ruby, Node.js) through JSON-RPC calls.

> [!IMPORTANT]
> This project is not affiliated with or endorsed by CenySlovensko. It is an independent implementation.

## Examples

```toml
[dependencies]
cenyslovensko_api = { version = "0.1.7", features = ["full"] }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

```rust
use cenyslovensko_api::{
    version::{adapters::http::HttpVersionSource, application::GetVersionUseCase},
    vendor::{adapters::http::HttpVendorSource, application::get_vendors_use_case::GetVendorsUseCase},
    web_client::WebClientConfig,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let web_client = WebClientConfig::new("https://api.cenyslovensko.sk/").build()?;

    let version_use_case = GetVersionUseCase::new(HttpVersionSource::new(web_client.clone(), "version"));
    let vendor_use_case = GetVendorsUseCase::new(HttpVendorSource::new(web_client, "vendor".to_string()));

    let version = version_use_case.execute().await?;
    let vendors = vendor_use_case.execute().await?;

    println!("API version: {version}");
    println!("Found {} vendors", vendors.len());

    Ok(())
}
```
