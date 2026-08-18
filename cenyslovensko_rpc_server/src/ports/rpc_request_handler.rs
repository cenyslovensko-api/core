use crate::domain::{RpcRequest, RpcResponse};
use std::future::Future;

pub trait RpcRequestHandler {
    fn handle_request(&self, request: RpcRequest) -> impl Future<Output = RpcResponse>;
}
