# CenySlovensko Version API

Rust API module for retrieving CenySlovensko API version data. The crate follows ports-and-adapters architecture and
uses `cenyslovensko_web_client` for HTTP configuration.

> [!NOTE]
> Exposes version retrieval through application/domain/ports/adapters layers for easier testing and future extension.

> [!IMPORTANT]
> This project is not affiliated with or endorsed by CenySlovensko. It is an independent implementation.

## Examples

```rust
use cenyslovensko_version_api::{adapters::http::HttpVersionSource, application::GetVersionUseCase};
use cenyslovensko_web_client::WebClientConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let web_client = WebClientConfig::new("https://api.cenyslovensko.sk/").build()?;
    let source = HttpVersionSource::new(web_client, "version");
    let use_case = GetVersionUseCase::new(source);

    let version = use_case.execute().await?;

    println!("API version: {version}");
    Ok(())
}
```
