use anyhow::{Context, Result, anyhow};
use cenyslovensko_api::web_client::{LogLevel, WebClient, WebClientConfig};
use std::env;
use std::time::Duration;

const DEFAULT_BASE_URI: &str = "https://api.cenyslovensko.sk/";
const DEFAULT_VERSION_PATH: &str = "api/version";
const DEFAULT_VENDOR_PATH: &str = "api/vendor-branch";
const DEFAULT_TIMEOUT_MS: u64 = 30_000;
const DEFAULT_CONNECT_TIMEOUT_MS: u64 = 10_000;

pub struct RpcServerConfig {
    pub web_client: WebClient,
    pub version_path: String,
    pub vendor_path: String,
}

impl RpcServerConfig {
    pub fn from_env() -> Result<Self> {
        let web_client = build_web_client_from_env()?;
        let version_path =
            env::var("CENYSLOVENSKO_VERSION_PATH").unwrap_or_else(|_| DEFAULT_VERSION_PATH.into());
        let vendor_path =
            env::var("CENYSLOVENSKO_VENDOR_PATH").unwrap_or_else(|_| DEFAULT_VENDOR_PATH.into());
        Ok(Self {
            web_client,
            version_path,
            vendor_path,
        })
    }
}

fn build_web_client_from_env() -> Result<WebClient> {
    let base_uri = env::var("CENYSLOVENSKO_BASE_URI").unwrap_or_else(|_| DEFAULT_BASE_URI.into());
    let timeout_ms = parse_u64_env("CENYSLOVENSKO_TIMEOUT_MS", DEFAULT_TIMEOUT_MS)?;
    let connect_timeout_ms = parse_u64_env(
        "CENYSLOVENSKO_CONNECT_TIMEOUT_MS",
        DEFAULT_CONNECT_TIMEOUT_MS,
    )?;
    let log_level = parse_log_level(env::var("CENYSLOVENSKO_LOG_LEVEL").ok().as_deref())?;
    let proxy = env::var("HTTPS_PROXY").ok();

    let mut config = WebClientConfig::new(base_uri)
        .set_timeout(Duration::from_millis(timeout_ms))
        .set_connect_timeout(Duration::from_millis(connect_timeout_ms))
        .set_log_level(log_level);

    if let Some(proxy) = proxy {
        config = config.set_proxy(proxy);
    }

    config.build().map_err(|error| anyhow!(error.to_string()))
}

fn parse_u64_env(variable: &str, default: u64) -> Result<u64> {
    match env::var(variable) {
        Ok(value) => value
            .parse::<u64>()
            .with_context(|| format!("{variable} must be a valid positive integer")),
        Err(_) => Ok(default),
    }
}

fn parse_log_level(value: Option<&str>) -> Result<LogLevel> {
    match value {
        None => Ok(LogLevel::Info),
        Some(level) if level.eq_ignore_ascii_case("error") => Ok(LogLevel::Error),
        Some(level) if level.eq_ignore_ascii_case("warn") => Ok(LogLevel::Warn),
        Some(level) if level.eq_ignore_ascii_case("info") => Ok(LogLevel::Info),
        Some(level) if level.eq_ignore_ascii_case("debug") => Ok(LogLevel::Debug),
        Some(level) if level.eq_ignore_ascii_case("trace") => Ok(LogLevel::Trace),
        Some(level) => Err(anyhow!(
            "Unsupported CENYSLOVENSKO_LOG_LEVEL value: {level}"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_known_log_levels() {
        assert_eq!(
            parse_log_level(Some("debug")).expect("debug level should parse"),
            LogLevel::Debug
        );
        assert_eq!(
            parse_log_level(Some("TRACE")).expect("trace level should parse"),
            LogLevel::Trace
        );
    }

    #[test]
    fn rejects_unknown_log_level() {
        let result = parse_log_level(Some("verbose"));
        assert!(result.is_err());
    }
}
