//! Vendor API client for CenySlovensko.
//!
//! The crate exposes domain models, a vendor source port, and an HTTP adapter that can be composed through the
//! application use case.
//!
//! # Examples
//!
//! ```no_run
//! use cenyslovensko_vendor_api::{
//!     adapters::http::HttpVendorSource,
//!     application::get_vendors_use_case::GetVendorsUseCase,
//! };
//! use cenyslovensko_web_client::WebClientConfig;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let web_client = WebClientConfig::new("https://api.cenyslovensko.sk/").build()?;
//! let source = HttpVendorSource::new(web_client, "vendor".to_string());
//! let use_case = GetVendorsUseCase::new(source);
//!
//! let vendors = use_case.execute().await?;
//! println!("Found {} vendors", vendors.len());
//! # Ok(())
//! # }
//! ```
//!
pub mod adapters;
pub mod application;
pub mod domain;
pub mod ports;
