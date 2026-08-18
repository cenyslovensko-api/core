use cenyslovensko_api::version::adapters::http::HttpVersionSource;
use cenyslovensko_api::version::application::GetVersionUseCase;
use cenyslovensko_api::web_client::WebClient;

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
    fn get_version(&self) -> impl Future<Output = Result<String, String>> {
        async move {
            self.use_case
                .execute()
                .await
                .map(|version| version.to_string())
                .map_err(|error| error.to_string())
        }
    }
}
