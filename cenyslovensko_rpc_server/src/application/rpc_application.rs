use crate::domain::{RpcRequest, RpcResponse, VENDOR_GET_METHOD, VERSION_GET_METHOD};
use crate::ports::{RpcRequestHandler, VendorGateway, VersionGateway};
use serde_json::json;

#[doc(hidden)]
pub struct MissingVersionGateway;
#[doc(hidden)]
pub struct MissingVendorGateway;

pub struct RpcApplication<TVersionGateway, TVendorGateway> {
    version_gateway: TVersionGateway,
    vendor_gateway: TVendorGateway,
}

pub struct RpcApplicationBuilder<
    TVersionGateway = MissingVersionGateway,
    TVendorGateway = MissingVendorGateway,
> {
    version_gateway: TVersionGateway,
    vendor_gateway: TVendorGateway,
}

impl RpcApplicationBuilder {
    pub fn new() -> Self {
        Self {
            version_gateway: MissingVersionGateway,
            vendor_gateway: MissingVendorGateway,
        }
    }
}

impl Default for RpcApplicationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl<TVersionGateway, TVendorGateway> RpcApplicationBuilder<TVersionGateway, TVendorGateway> {
    pub fn version_gateway<TNewVersionGateway: VersionGateway>(
        self,
        version_gateway: TNewVersionGateway,
    ) -> RpcApplicationBuilder<TNewVersionGateway, TVendorGateway> {
        RpcApplicationBuilder {
            version_gateway,
            vendor_gateway: self.vendor_gateway,
        }
    }

    pub fn vendor_gateway<TNewVendorGateway: VendorGateway>(
        self,
        vendor_gateway: TNewVendorGateway,
    ) -> RpcApplicationBuilder<TVersionGateway, TNewVendorGateway> {
        RpcApplicationBuilder {
            version_gateway: self.version_gateway,
            vendor_gateway,
        }
    }
}

impl<TVersionGateway, TVendorGateway> RpcApplicationBuilder<TVersionGateway, TVendorGateway>
where
    TVersionGateway: VersionGateway,
    TVendorGateway: VendorGateway,
{
    pub fn build(self) -> RpcApplication<TVersionGateway, TVendorGateway> {
        RpcApplication {
            version_gateway: self.version_gateway,
            vendor_gateway: self.vendor_gateway,
        }
    }
}

impl<TVersionGateway, TVendorGateway> RpcApplication<TVersionGateway, TVendorGateway>
where
    TVersionGateway: VersionGateway,
    TVendorGateway: VendorGateway,
{
    pub fn new(version_gateway: TVersionGateway, vendor_gateway: TVendorGateway) -> Self {
        RpcApplicationBuilder::new()
            .version_gateway(version_gateway)
            .vendor_gateway(vendor_gateway)
            .build()
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
        let app = RpcApplicationBuilder::new()
            .version_gateway(FakeVersionGateway {
                result: Ok("0.1.370".into()),
            })
            .vendor_gateway(FakeVendorGateway { result: Ok(vec![]) })
            .build();
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
        let app = RpcApplicationBuilder::new()
            .version_gateway(FakeVersionGateway {
                result: Ok("0.1.370".into()),
            })
            .vendor_gateway(FakeVendorGateway {
                result: Ok(vec![Vendor::new(
                    "branch_1".into(),
                    "Main Branch".into(),
                    VendorAddress::new("Bratislava".into(), "123".into()),
                    "company_1".into(),
                    VendorLocation::new(48.8566, 2.3522),
                )]),
            })
            .build();
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
        let app = RpcApplicationBuilder::new()
            .version_gateway(FakeVersionGateway {
                result: Ok("0.1.370".into()),
            })
            .vendor_gateway(FakeVendorGateway {
                result: Err(VendorError::Unavailable("vendor unavailable".into())),
            })
            .build();
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
        let app = RpcApplicationBuilder::new()
            .version_gateway(FakeVersionGateway {
                result: Ok("0.1.370".into()),
            })
            .vendor_gateway(FakeVendorGateway { result: Ok(vec![]) })
            .build();
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

    #[tokio::test]
    async fn builder_constructs_application() {
        let app = RpcApplicationBuilder::new()
            .vendor_gateway(FakeVendorGateway { result: Ok(vec![]) })
            .version_gateway(FakeVersionGateway {
                result: Ok("0.1.370".into()),
            })
            .build();
        let request = RpcRequest {
            id: Value::from(1),
            method: VERSION_GET_METHOD.into(),
            params: None,
        };

        let response = app.handle_request(request).await;

        assert_eq!(response.result, Some(json!({ "version": "0.1.370" })));
    }
}
