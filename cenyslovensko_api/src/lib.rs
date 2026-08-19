//! Umbrella crate for the CenySlovensko Rust API.
//!
//! Enable only the feature-gated modules you need:
//! - `web_client` for shared HTTP client configuration
//! - `version` for the version API client
//! - `vendor` for the vendor API client
//! - `product` for product categories and product prices API clients
//! - `full` for all of the above
//!
//! # Examples
//!
//! ```no_run
//! # #[cfg(feature = "full")]
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! use cenyslovensko_api::{
//!     version::{adapters::http::HttpVersionSource, application::GetVersionUseCase},
//!     vendor::{adapters::http::HttpVendorSource, application::get_vendors_use_case::GetVendorsUseCase},
//!     web_client::WebClientConfig,
//! };
//!
//! let web_client = WebClientConfig::new("https://api.cenyslovensko.sk/").build()?;
//! let version_use_case = GetVersionUseCase::new(HttpVersionSource::new(web_client.clone(), "version"));
//! let vendor_use_case = GetVendorsUseCase::new(HttpVendorSource::new(web_client, "vendor".to_string()));
//!
//! let version = version_use_case.execute().await?;
//! let vendors = vendor_use_case.execute().await?;
//!
//! assert!(!version.to_string().is_empty());
//! println!("Found {} vendors", vendors.len());
//! # Ok(())
//! # }
//! ```
//!
/// Configurable HTTP web client (base URI, headers, timeouts, proxy, log level).
///
/// Enabled by feature `web_client`.
#[cfg(feature = "web_client")]
pub use cenyslovensko_web_client as web_client;

/// Version API - fetch the current API version over HTTP.
///
/// Enabled by feature `version` (implies `web_client`).
#[cfg(feature = "version")]
pub use cenyslovensko_version_api as version;

/// Vendor API - fetch the current vendor information over HTTP.
///
/// Enabled by feature `vendor` (implies `web_client`).
#[cfg(feature = "vendor")]
pub use cenyslovensko_vendor_api as vendor;

/// Product API - fetch product categories and current-day product prices over HTTP.
///
/// Enabled by feature `product` (implies `web_client`).
#[cfg(feature = "product")]
pub use cenyslovensko_product_api as product;
