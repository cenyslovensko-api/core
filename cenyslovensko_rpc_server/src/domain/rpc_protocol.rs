use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const VERSION_GET_METHOD: &str = "version.get";
pub const VENDOR_GET_METHOD: &str = "vendor.get";
pub const PRODUCT_PRICES_CURRENT_DAY_GET_METHOD: &str = "product-prices.current-day.get";

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct RpcRequest {
    pub id: Value,
    pub method: String,
    #[serde(default)]
    pub params: Option<Value>,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct RpcError {
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct RpcResponse {
    pub jsonrpc: &'static str,
    pub id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<RpcError>,
}

impl RpcResponse {
    pub fn success(id: Value, result: Value) -> Self {
        Self {
            jsonrpc: "2.0",
            id,
            result: Some(result),
            error: None,
        }
    }

    pub fn error(id: Value, code: i32, message: impl Into<String>) -> Self {
        Self {
            jsonrpc: "2.0",
            id,
            result: None,
            error: Some(RpcError {
                code,
                message: message.into(),
            }),
        }
    }

    pub fn parse_error() -> Self {
        Self::error(Value::Null, -32700, "Parse error")
    }

    pub fn method_not_found(id: Value) -> Self {
        Self::error(id, -32601, "Method not found")
    }

    pub fn invalid_params(id: Value, message: impl Into<String>) -> Self {
        Self::error(id, -32602, message)
    }

    pub fn internal_error(id: Value, message: impl Into<String>) -> Self {
        Self::error(id, -32000, message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_parse_error_response() {
        let response = RpcResponse::parse_error();
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, Value::Null);
        assert!(response.result.is_none());
        assert_eq!(response.error.expect("error should be set").code, -32700);
    }
}
