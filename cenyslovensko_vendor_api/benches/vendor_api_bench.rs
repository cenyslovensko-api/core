use cenyslovensko_vendor_api::adapters::http::HttpVendorSource;
use cenyslovensko_vendor_api::ports::vendor_source::VendorSource;
use cenyslovensko_web_client::WebClientConfig;
use criterion::{Criterion, criterion_group, criterion_main};
use httpmock::Method::GET;
use httpmock::MockServer;
use std::hint::black_box;
use tokio::runtime::Runtime;

fn bench_web_client_resolve_url(c: &mut Criterion) {
    let client = WebClientConfig::new("https://api.cenyslovensko.sk/")
        .build()
        .unwrap();

    c.bench_function("web_client_resolve_url", |b| {
        b.iter(|| client.resolve_url(black_box("version")).unwrap())
    });
}

fn bench_http_vendor_source_get_vendor(c: &mut Criterion) {
    let server = MockServer::start();
    let _mock = server.mock(|when, then| {
        when.method(GET).path("/vendor");
        then.status(200)
            .header("content-type", "application/json")
            .body(
                r#"[
                {
                    "branchId":"branch_1",
                    "branchName":"Main Branch",
                    "address":{"city":"Bratislava","streetNumber":"123"},
                    "companyId":"company_1",
                    "location":{"lat":48.8566,"lng":2.3522}
                }
                ]"#,
            );
    });

    let client = WebClientConfig::new(server.base_url()).build().unwrap();
    let source = HttpVendorSource::new(client, "vendor".to_string());
    let rt = Runtime::new().unwrap();

    c.bench_function("http_vendor_source_get_vendor", |b| {
        b.to_async(&rt).iter(|| source.get_vendor())
    });
}

criterion_group!(
    benches,
    bench_web_client_resolve_url,
    bench_http_vendor_source_get_vendor,
);

criterion_main!(benches);
