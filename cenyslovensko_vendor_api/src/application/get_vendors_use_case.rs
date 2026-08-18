use crate::domain::vendor::Vendor;
use crate::domain::vendor_error::VendorError;
use crate::ports::vendor_source::VendorSource;
pub struct GetVendorsUseCase<TVendorSource>
where
    TVendorSource: VendorSource,
{
    vendor_source: TVendorSource,
}

impl<TVendorSource> GetVendorsUseCase<TVendorSource>
where
    TVendorSource: VendorSource,
{
    pub fn new(vendor_source: TVendorSource) -> Self {
        Self { vendor_source }
    }

    pub async fn execute(&self) -> Result<Vec<Vendor>, VendorError> {
        self.vendor_source.get_vendor().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::vendor::{Vendor, VendorAddress, VendorLocation};

    #[derive(Clone)]
    struct FakeVendorSource {
        result: Result<Vec<Vendor>, VendorError>,
    }

    impl VendorSource for FakeVendorSource {
        async fn get_vendor(&self) -> Result<Vec<Vendor>, VendorError> {
            self.result.clone()
        }
    }

    #[tokio::test]
    async fn returns_vendors_from_source() {
        let use_case = GetVendorsUseCase::new(FakeVendorSource {
            result: Ok(vec![Vendor::new(
                "branch_1".into(),
                "Main Branch".into(),
                VendorAddress::new("Bratislava".into(), "123".into()),
                "company_1".into(),
                VendorLocation::new(48.8566, 2.3522),
            )]),
        });

        let result = use_case.execute().await;

        assert_eq!(
            result,
            Ok(vec![Vendor::new(
                "branch_1".into(),
                "Main Branch".into(),
                VendorAddress::new("Bratislava".into(), "123".into()),
                "company_1".into(),
                VendorLocation::new(48.8566, 2.3522),
            )])
        );
    }

    #[tokio::test]
    async fn returns_error_from_source() {
        let use_case = GetVendorsUseCase::new(FakeVendorSource {
            result: Err(VendorError::NotFound),
        });

        let result = use_case.execute().await;

        assert_eq!(result, Err(VendorError::NotFound));
    }
}
