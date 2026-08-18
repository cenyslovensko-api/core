use crate::domain::{RpcRequest, RpcResponse, VENDOR_GET_METHOD, VERSION_GET_METHOD};
use crate::ports::{RpcRequestHandler, VendorGateway, VersionGateway};
use serde_json::json;

pub struct RpcApplication<TVersionGateway, TVendorGateway>
where
    TVersionGateway: VersionGateway,
    TVendorGateway: VendorGateway,
{
    version_gateway: TVersionGateway,
    vendor_gateway: TVendorGateway,
}

impl<TVersionGateway, TVendorGateway> RpcApplication<TVersionGateway, TVendorGateway>
where
    TVersionGateway: VersionGateway,
    TVendorGateway: VendorGateway,
{
    pub fn new(version_gateway: TVersionGateway, vendor_gateway: TVendorGateway) -> Self {
        Self {
            version_gateway,
            vendor_gateway,
        }
    }
}

impl<TVersionGateway, TVendorGateway> RpcRequestHandler
    for RpcApplication<TVersionGateway, TVendorGateway>
where
    TVersionGateway: VersionGateway,
    TVendorGateway: VendorGateway,
{
    async fn handle_request(&self, request: RpcRequest) -> RpcResponse {
        match request.method.as_str() {
            VERSION_GET_METHOD => match self.version_gateway.get_version().await {
                Ok(version) => RpcResponse::success(request.id, json!({ "version": version })),
                Err(error) => RpcResponse::internal_error(request.id, error),
            },
            VENDOR_GET_METHOD => match self.vendor_gateway.get_vendor().await {
                Ok(vendors) => RpcResponse::success(request.id, json!({ "vendors": vendors })),
                Err(error) => RpcResponse::internal_error(request.id, error.to_string()),
            },
            _ => RpcResponse::method_not_found(request.id),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::{VendorGateway, VersionGateway};
    use cenyslovensko_api::vendor::domain::vendor::{Vendor, VendorAddress, VendorLocation};
    use cenyslovensko_api::vendor::domain::vendor_error::VendorError;
    use serde_json::Value;

    #[derive(Clone)]
    struct FakeVersionGateway {
        result: Result<String, String>,
    }

    impl VersionGateway for FakeVersionGateway {
        async fn get_version(&self) -> Result<String, String> {
            self.result.clone()
        }
    }

    #[derive(Clone)]
    struct FakeVendorGateway {
        result: Result<Vec<Vendor>, VendorError>,
    }

    impl VendorGateway for FakeVendorGateway {
        async fn get_vendor(&self) -> Result<Vec<Vendor>, VendorError> {
            self.result.clone()
        }
    }

    #[tokio::test]
    async fn returns_version_for_version_get_method() {
        let app = RpcApplication::new(
            FakeVersionGateway {
                result: Ok("0.1.370".into()),
            },
            FakeVendorGateway { result: Ok(vec![]) },
        );
        let request = RpcRequest {
            id: Value::from(1),
            method: VERSION_GET_METHOD.into(),
            params: None,
        };

        let response = app.handle_request(request).await;

        assert_eq!(response.error, None);
        assert_eq!(response.result, Some(json!({ "version": "0.1.370" })));
    }

    #[tokio::test]
    async fn returns_vendors_for_vendor_get_method() {
        let app = RpcApplication::new(
            FakeVersionGateway {
                result: Ok("0.1.370".into()),
            },
            FakeVendorGateway {
                result: Ok(vec![Vendor::new(
                    "branch_1".into(),
                    "Main Branch".into(),
                    VendorAddress::new("Bratislava".into(), "123".into()),
                    "company_1".into(),
                    VendorLocation::new(48.8566, 2.3522),
                )]),
            },
        );
        let request = RpcRequest {
            id: Value::from(1),
            method: VENDOR_GET_METHOD.into(),
            params: None,
        };

        let response = app.handle_request(request).await;

        assert_eq!(response.error, None);
        assert_eq!(
            response.result,
            Some(json!({
                "vendors": [{
                    "branch_id": "branch_1",
                    "branch_name": "Main Branch",
                    "address": {
                        "city": "Bratislava",
                        "street_number": "123"
                    },
                    "company_id": "company_1",
                    "location": {
                        "lat": 48.8566,
                        "lng": 2.3522
                    }
                }]
            }))
        );
    }

    #[tokio::test]
    async fn returns_internal_error_for_vendor_failure() {
        let app = RpcApplication::new(
            FakeVersionGateway {
                result: Ok("0.1.370".into()),
            },
            FakeVendorGateway {
                result: Err(VendorError::Unavailable("vendor unavailable".into())),
            },
        );
        let request = RpcRequest {
            id: Value::from(1),
            method: VENDOR_GET_METHOD.into(),
            params: None,
        };

        let response = app.handle_request(request).await;

        assert!(response.result.is_none());
        assert_eq!(
            response.error.expect("error should be present").message,
            "Vendor source is unavailable: vendor unavailable"
        );
    }

    #[tokio::test]
    async fn returns_method_not_found_for_unknown_method() {
        let app = RpcApplication::new(
            FakeVersionGateway {
                result: Ok("0.1.370".into()),
            },
            FakeVendorGateway { result: Ok(vec![]) },
        );
        let request = RpcRequest {
            id: Value::from(1),
            method: "unknown.method".into(),
            params: None,
        };

        let response = app.handle_request(request).await;

        assert!(response.result.is_none());
        assert_eq!(
            response.error.expect("error should be present").code,
            -32601
        );
    }
}
