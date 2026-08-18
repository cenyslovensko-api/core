use crate::ports::vendor_gateway::VendorGateway;
use cenyslovensko_api::vendor::adapters::http::HttpVendorSource;
use cenyslovensko_api::vendor::application::get_vendors_use_case::GetVendorsUseCase;
use cenyslovensko_api::vendor::domain::vendor::Vendor;
use cenyslovensko_api::vendor::domain::vendor_error::VendorError;
use cenyslovensko_api::web_client::WebClient;

pub struct VendorApiGateway {
    use_case: GetVendorsUseCase<HttpVendorSource>,
}

impl VendorApiGateway {
    pub fn new(web_client: WebClient, vendor_path: impl Into<String>) -> Self {
        Self {
            use_case: GetVendorsUseCase::new(HttpVendorSource::new(web_client, vendor_path.into())),
        }
    }
}

impl VendorGateway for VendorApiGateway {
    async fn get_vendor(&self) -> Result<Vec<Vendor>, VendorError> {
        self.use_case.execute().await
    }
}
