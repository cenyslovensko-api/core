use cenyslovensko_web_client::WebClient;
use serde::Deserialize;
use std::future::Future;

use crate::domain::{Version, VersionError};
use crate::ports::VersionSource;

pub struct HttpVersionSource {
    web_client: WebClient,
    version_path: String,
}

impl HttpVersionSource {
    pub fn new(web_client: WebClient, version_path: impl Into<String>) -> Self {
        Self {
            web_client,
            version_path: version_path.into(),
        }
    }
}

#[derive(Deserialize)]
struct VersionResponse {
    version: String,
}

impl VersionSource for HttpVersionSource {
    fn get_version(&self) -> impl Future<Output = Result<Version, VersionError>> + Send {
        async move {
            let endpoint = self
                .web_client
                .resolve_url(&self.version_path)
                .map_err(|error| VersionError::Unavailable(error.to_string()))?;

            let response = self
                .web_client
                .client()
                .get(endpoint)
                .send()
                .await
                .map_err(|error| VersionError::Unavailable(error.to_string()))?;

            if response.status() == reqwest::StatusCode::NOT_FOUND {
                return Err(VersionError::NotFound);
            }

            let response = response
                .error_for_status()
                .map_err(|error| VersionError::Unavailable(error.to_string()))?;

            let response: VersionResponse = response
                .json()
                .await
                .map_err(|error| VersionError::InvalidResponse(error.to_string()))?;

            Ok(Version::new(response.version))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cenyslovensko_web_client::WebClientConfig;
    use httpmock::Method::GET;
    use httpmock::MockServer;

    #[tokio::test]
    async fn returns_version_from_valid_json_response() {
        let server = MockServer::start();
        let _mock = server.mock(|when, then| {
            when.method(GET).path("/version");
            then.status(200)
                .header("content-type", "application/json")
                .body(r#"{"version":"0.1.370"}"#);
        });

        let web_client = WebClientConfig::new(server.base_url())
            .build()
            .expect("web client should build");
        let source = HttpVersionSource::new(web_client, "version");

        let result = source.get_version().await;

        assert_eq!(result, Ok(Version::new("0.1.370".into())));
    }

    #[tokio::test]
    async fn returns_not_found_for_404_response() {
        let server = MockServer::start();
        let _mock = server.mock(|when, then| {
            when.method(GET).path("/version");
            then.status(404);
        });

        let web_client = WebClientConfig::new(server.base_url())
            .build()
            .expect("web client should build");
        let source = HttpVersionSource::new(web_client, "version");

        let result = source.get_version().await;

        assert_eq!(result, Err(VersionError::NotFound));
    }

    #[tokio::test]
    async fn returns_invalid_response_for_non_json_body() {
        let server = MockServer::start();
        let _mock = server.mock(|when, then| {
            when.method(GET).path("/version");
            then.status(200)
                .header("content-type", "application/json")
                .body(r#"{"missing_version":"0.1.370"}"#);
        });

        let web_client = WebClientConfig::new(server.base_url())
            .build()
            .expect("web client should build");
        let source = HttpVersionSource::new(web_client, "version");

        let result = source.get_version().await;

        assert!(matches!(result, Err(VersionError::InvalidResponse(_))));
    }

    #[tokio::test]
    async fn returns_unavailable_for_server_errors() {
        let server = MockServer::start();
        let _mock = server.mock(|when, then| {
            when.method(GET).path("/version");
            then.status(500);
        });

        let web_client = WebClientConfig::new(server.base_url())
            .build()
            .expect("web client should build");
        let source = HttpVersionSource::new(web_client, "version");

        let result = source.get_version().await;

        assert!(matches!(result, Err(VersionError::Unavailable(_))));
    }
}
