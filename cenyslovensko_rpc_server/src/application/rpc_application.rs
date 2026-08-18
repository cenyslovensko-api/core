use crate::domain::{RpcRequest, RpcResponse, VERSION_GET_METHOD};
use crate::ports::{RpcRequestHandler, VersionGateway};
use serde_json::json;

pub struct RpcApplication<TVersionGateway>
where
    TVersionGateway: VersionGateway,
{
    version_gateway: TVersionGateway,
}

impl<TVersionGateway> RpcApplication<TVersionGateway>
where
    TVersionGateway: VersionGateway,
{
    pub fn new(version_gateway: TVersionGateway) -> Self {
        Self { version_gateway }
    }
}

impl<TVersionGateway> RpcRequestHandler for RpcApplication<TVersionGateway>
where
    TVersionGateway: VersionGateway,
{
    async fn handle_request(&self, request: RpcRequest) -> RpcResponse {
        match request.method.as_str() {
            VERSION_GET_METHOD => match self.version_gateway.get_version().await {
                Ok(version) => RpcResponse::success(request.id, json!({ "version": version })),
                Err(error) => RpcResponse::internal_error(request.id, error),
            },
            _ => RpcResponse::method_not_found(request.id),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::VersionGateway;
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

    #[tokio::test]
    async fn returns_version_for_version_get_method() {
        let app = RpcApplication::new(FakeVersionGateway {
            result: Ok("0.1.370".into()),
        });
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
    async fn returns_method_not_found_for_unknown_method() {
        let app = RpcApplication::new(FakeVersionGateway {
            result: Ok("0.1.370".into()),
        });
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
