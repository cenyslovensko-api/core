use cenyslovensko_api::vendor::domain::vendor::Vendor;
use cenyslovensko_api::vendor::domain::vendor_error::VendorError;
use cenyslovensko_rpc_server::adapters::stdio_rpc_server;
use cenyslovensko_rpc_server::application::RpcApplicationBuilder;
use cenyslovensko_rpc_server::domain::{
    RpcRequest, RpcResponse, VENDOR_GET_METHOD, VERSION_GET_METHOD,
};
use cenyslovensko_rpc_server::ports::{RpcRequestHandler, VendorGateway, VersionGateway};
use criterion::{Criterion, criterion_group, criterion_main};
use serde_json::{Value, json};
use std::hint::black_box;
use std::time::{Duration, Instant};

struct FakeVersionGateway;

impl VersionGateway for FakeVersionGateway {
    async fn get_version(&self) -> Result<String, String> {
        Ok("0.1.370".into())
    }
}

struct FakeVendorGateway;

impl VendorGateway for FakeVendorGateway {
    async fn get_vendor(&self) -> Result<Vec<Vendor>, VendorError> {
        Ok(vec![])
    }
}

fn build_app() -> impl RpcRequestHandler {
    RpcApplicationBuilder::new()
        .version_gateway(FakeVersionGateway)
        .vendor_gateway(FakeVendorGateway)
        .build()
}

fn bench_handle_version_get(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let app = RpcApplicationBuilder::new()
        .version_gateway(FakeVersionGateway)
        .vendor_gateway(FakeVendorGateway)
        .build();
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
    let app = RpcApplicationBuilder::new()
        .version_gateway(FakeVersionGateway)
        .vendor_gateway(FakeVendorGateway)
        .build();
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

fn bench_handle_vendor_get(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let app = RpcApplicationBuilder::new()
        .version_gateway(FakeVersionGateway)
        .vendor_gateway(FakeVendorGateway)
        .build();
    let request = RpcRequest {
        id: Value::from(1),
        method: VENDOR_GET_METHOD.into(),
        params: None,
    };

    c.bench_function("handle_vendor_get", |b| {
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

fn bench_rpc_server_startup_time(c: &mut Criterion) {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    c.bench_function("rpc_server_startup_time", |b| {
        b.iter_custom(|iters| {
            let mut total = Duration::ZERO;

            for _ in 0..iters {
                let local = tokio::task::LocalSet::new();
                total += rt.block_on(local.run_until(async {
                    let app = build_app();
                    let (client, server) = tokio::io::duplex(64);
                    let task = tokio::task::spawn_local(async move {
                        stdio_rpc_server::run_with_io(&app, server, tokio::io::sink()).await
                    });

                    let started_at = Instant::now();
                    tokio::task::yield_now().await;
                    let startup_elapsed = started_at.elapsed();

                    drop(client);
                    task.await.unwrap().unwrap();

                    startup_elapsed
                }));
            }

            total
        });
    });
}

fn bench_rpc_server_shutdown_time(c: &mut Criterion) {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    c.bench_function("rpc_server_shutdown_time", |b| {
        b.iter_custom(|iters| {
            let mut total = Duration::ZERO;

            for _ in 0..iters {
                let local = tokio::task::LocalSet::new();
                total += rt.block_on(local.run_until(async {
                    let app = build_app();
                    let (client, server) = tokio::io::duplex(64);
                    let task = tokio::task::spawn_local(async move {
                        stdio_rpc_server::run_with_io(&app, server, tokio::io::sink()).await
                    });

                    tokio::task::yield_now().await;

                    let shutdown_started_at = Instant::now();
                    drop(client);
                    task.await.unwrap().unwrap();

                    shutdown_started_at.elapsed()
                }));
            }

            total
        });
    });
}

criterion_group!(
    benches,
    bench_handle_version_get,
    bench_handle_vendor_get,
    bench_handle_unknown_method,
    bench_rpc_request_deserialize,
    bench_rpc_response_serialize,
    bench_rpc_server_startup_time,
    bench_rpc_server_shutdown_time,
);
criterion_main!(benches);
