use crate::domain::vendor::Vendor;
use crate::domain::vendor_error::VendorError;

pub trait VendorSource {
    fn get_vendor(&self) -> impl Future<Output = Result<Vec<Vendor>, VendorError>> + Send;
}
