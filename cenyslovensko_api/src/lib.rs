/// Configurable HTTP web client (base URI, headers, timeouts, proxy, log level).
///
/// Enabled by feature `web_client`.
#[cfg(feature = "web_client")]
pub use cenyslovensko_web_client as web_client;

/// Version API — fetch the current API version over HTTP.
///
/// Enabled by feature `version` (implies `web_client`).
#[cfg(feature = "version")]
pub use cenyslovensko_version_api as version;
