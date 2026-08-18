use cenyslovensko_rpc_server::application::RpcApplication;
use cenyslovensko_rpc_server::domain::{RpcRequest, RpcResponse, VERSION_GET_METHOD};
use cenyslovensko_rpc_server::ports::{RpcRequestHandler, VersionGateway};
use criterion::{Criterion, criterion_group, criterion_main};
use serde_json::{Value, json};
use std::future::Future;
use std::hint::black_box;

struct FakeVersionGateway;

impl VersionGateway for FakeVersionGateway {
    fn get_version(&self) -> impl Future<Output = Result<String, String>> {
        async { Ok("0.1.370".into()) }
    }
}

fn bench_handle_version_get(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let app = RpcApplication::new(FakeVersionGateway);
    let request = RpcRequest {
        id: Value::from(1),
        method: VERSION_GET_METHOD.into(),
        params: None,
    };

    c.bench_function("handle_version_get", |b| {
        b.to_async(&rt)
            .iter(|| app.handle_request(black_box(request.clone())))
    });
}

fn bench_handle_unknown_method(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let app = RpcApplication::new(FakeVersionGateway);
    let request = RpcRequest {
        id: Value::from(1),
        method: "unknown.method".into(),
        params: None,
    };

    c.bench_function("handle_unknown_method", |b| {
        b.to_async(&rt)
            .iter(|| app.handle_request(black_box(request.clone())))
    });
}

fn bench_rpc_request_deserialize(c: &mut Criterion) {
    let json_str = r#"{"id":1,"method":"version.get","params":null}"#;

    c.bench_function("rpc_request_deserialize", |b| {
        b.iter(|| serde_json::from_str::<RpcRequest>(black_box(json_str)).unwrap())
    });
}

fn bench_rpc_response_serialize(c: &mut Criterion) {
    let response = RpcResponse::success(Value::from(1), json!({"version": "0.1.370"}));

    c.bench_function("rpc_response_serialize", |b| {
        b.iter(|| serde_json::to_string(black_box(&response)).unwrap())
    });
}

criterion_group!(
    benches,
    bench_handle_version_get,
    bench_handle_unknown_method,
    bench_rpc_request_deserialize,
    bench_rpc_response_serialize,
);
criterion_main!(benches);
