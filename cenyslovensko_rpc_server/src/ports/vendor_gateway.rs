use cenyslovensko_api::vendor::domain::vendor::Vendor;
use cenyslovensko_api::vendor::domain::vendor_error::VendorError;

pub trait VendorGateway {
    fn get_vendor(&self) -> impl Future<Output = Result<Vec<Vendor>, VendorError>>;
}
