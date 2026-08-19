//! Version API client for CenySlovensko.
//!
//! The crate keeps the transport adapter separate from the application use case so the version source can be swapped
//! in tests or alternative integrations.
//!
//! # Examples
//!
//! ```no_run
//! use cenyslovensko_version_api::{adapters::http::HttpVersionSource, application::GetVersionUseCase};
//! use cenyslovensko_web_client::WebClientConfig;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let web_client = WebClientConfig::new("https://api.cenyslovensko.sk/").build()?;
//! let source = HttpVersionSource::new(web_client, "version");
//! let use_case = GetVersionUseCase::new(source);
//!
//! let version = use_case.execute().await?;
//! assert!(!version.to_string().is_empty());
//! # Ok(())
//! # }
//! ```
//!
pub mod adapters;
pub mod application;
pub mod domain;
pub mod ports;
