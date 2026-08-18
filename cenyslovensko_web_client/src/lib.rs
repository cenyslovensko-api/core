use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl Default for LogLevel {
    fn default() -> Self {
        Self::Info
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WebClientError {
    InvalidBaseUri(String),
    InvalidHeaderName(String),
    InvalidHeaderValue(String),
    InvalidProxy(String),
    ClientBuild(String),
    InvalidPath(String),
}

impl Display for WebClientError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidBaseUri(value) => write!(formatter, "Invalid base URI: {value}"),
            Self::InvalidHeaderName(value) => write!(formatter, "Invalid header name: {value}"),
            Self::InvalidHeaderValue(value) => write!(formatter, "Invalid header value: {value}"),
            Self::InvalidProxy(value) => write!(formatter, "Invalid proxy: {value}"),
            Self::ClientBuild(value) => write!(formatter, "Unable to build web client: {value}"),
            Self::InvalidPath(value) => write!(formatter, "Invalid path for base URI: {value}"),
        }
    }
}

impl Error for WebClientError {}

#[derive(Debug, Clone)]
pub struct WebClientConfig {
    base_uri: String,
    default_headers: HeaderMap,
    timeout: Duration,
    connect_timeout: Duration,
    proxy: Option<String>,
    log_level: LogLevel,
}

impl WebClientConfig {
    pub fn new(base_uri: impl Into<String>) -> Self {
        Self {
            base_uri: base_uri.into(),
            default_headers: HeaderMap::new(),
            timeout: Duration::from_secs(30),
            connect_timeout: Duration::from_secs(10),
            proxy: None,
            log_level: LogLevel::default(),
        }
    }

    pub fn set_base_uri(mut self, base_uri: impl Into<String>) -> Self {
        self.base_uri = base_uri.into();
        self
    }

    pub fn set_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn set_connect_timeout(mut self, connect_timeout: Duration) -> Self {
        self.connect_timeout = connect_timeout;
        self
    }

    pub fn set_proxy(mut self, proxy: impl Into<String>) -> Self {
        self.proxy = Some(proxy.into());
        self
    }

    pub fn set_log_level(mut self, log_level: LogLevel) -> Self {
        self.log_level = log_level;
        self
    }

    pub fn set_default_headers(mut self, headers: HeaderMap) -> Self {
        self.default_headers = headers;
        self
    }

    pub fn add_default_header(mut self, name: &str, value: &str) -> Result<Self, WebClientError> {
        let name = HeaderName::from_bytes(name.as_bytes())
            .map_err(|_| WebClientError::InvalidHeaderName(name.into()))?;
        let value = HeaderValue::from_str(value)
            .map_err(|_| WebClientError::InvalidHeaderValue(value.into()))?;
        self.default_headers.insert(name, value);
        Ok(self)
    }

    pub fn build(self) -> Result<WebClient, WebClientError> {
        let base_uri = reqwest::Url::parse(&self.base_uri)
            .map_err(|_| WebClientError::InvalidBaseUri(self.base_uri.clone()))?;

        let mut builder = reqwest::Client::builder()
            .default_headers(self.default_headers)
            .timeout(self.timeout)
            .connect_timeout(self.connect_timeout);

        if let Some(proxy) = &self.proxy {
            let parsed_proxy = reqwest::Proxy::all(proxy)
                .map_err(|_| WebClientError::InvalidProxy(proxy.clone()))?;
            builder = builder.proxy(parsed_proxy);
        }

        if matches!(self.log_level, LogLevel::Debug | LogLevel::Trace) {
            builder = builder.connection_verbose(true);
        }

        let client = builder
            .build()
            .map_err(|error| WebClientError::ClientBuild(error.to_string()))?;

        Ok(WebClient {
            client,
            base_uri,
            proxy: self.proxy,
            log_level: self.log_level,
        })
    }
}

#[derive(Debug, Clone)]
pub struct WebClient {
    client: reqwest::Client,
    base_uri: reqwest::Url,
    proxy: Option<String>,
    log_level: LogLevel,
}

impl WebClient {
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    pub fn base_uri(&self) -> &reqwest::Url {
        &self.base_uri
    }

    pub fn proxy(&self) -> Option<&str> {
        self.proxy.as_deref()
    }

    pub fn log_level(&self) -> LogLevel {
        self.log_level
    }

    pub fn resolve_url(&self, path: &str) -> Result<reqwest::Url, WebClientError> {
        self.base_uri
            .join(path)
            .map_err(|_| WebClientError::InvalidPath(path.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_client_and_resolves_path() {
        let web_client = WebClientConfig::new("https://api.cenyslovensko.sk/")
            .set_timeout(Duration::from_secs(15))
            .build()
            .expect("web client should build");

        let endpoint = web_client
            .resolve_url("version")
            .expect("path should resolve");

        assert_eq!(endpoint.as_str(), "https://api.cenyslovensko.sk/version");
    }

    #[test]
    fn fails_for_invalid_base_uri() {
        let result = WebClientConfig::new("not-a-url").build();

        assert!(matches!(result, Err(WebClientError::InvalidBaseUri(_))));
    }

    #[test]
    fn fails_for_invalid_proxy() {
        let result = WebClientConfig::new("https://api.cenyslovensko.sk/")
            .set_proxy("://invalid-proxy")
            .build();

        assert!(matches!(result, Err(WebClientError::InvalidProxy(_))));
    }

    #[test]
    fn fails_for_invalid_header_name() {
        let result = WebClientConfig::new("https://api.cenyslovensko.sk/")
            .add_default_header("invalid header", "value");

        assert!(matches!(result, Err(WebClientError::InvalidHeaderName(_))));
    }

    #[test]
    fn preserves_configured_proxy_and_log_level() {
        let web_client = WebClientConfig::new("https://api.cenyslovensko.sk/")
            .set_proxy("http://localhost:3128")
            .set_log_level(LogLevel::Debug)
            .build()
            .expect("web client should build");

        assert_eq!(web_client.proxy(), Some("http://localhost:3128"));
        assert_eq!(web_client.log_level(), LogLevel::Debug);
    }
}
