use cenyslovensko_version_api::adapters::http::HttpVersionSource;
use cenyslovensko_version_api::application::GetVersionUseCase;
use cenyslovensko_web_client::WebClient;

use crate::ports::VersionGateway;

pub struct VersionApiGateway {
    use_case: GetVersionUseCase<HttpVersionSource>,
}

impl VersionApiGateway {
    pub fn new(web_client: WebClient, version_path: impl Into<String>) -> Self {
        Self {
            use_case: GetVersionUseCase::new(HttpVersionSource::new(web_client, version_path)),
        }
    }
}

impl VersionGateway for VersionApiGateway {
    fn get_version(&self) -> impl std::future::Future<Output = Result<String, String>> {
        async move {
            self.use_case
                .execute()
                .await
                .map(|version| version.to_string())
                .map_err(|error| error.to_string())
        }
    }
}
