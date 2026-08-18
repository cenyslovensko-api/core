use cenyslovensko_api::web_client::WebClientConfig;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn bench_web_client_build(c: &mut Criterion) {
    c.bench_function("web_client_build", |b| {
        b.iter(|| {
            WebClientConfig::new(black_box("https://api.cenyslovensko.sk/"))
                .build()
                .unwrap()
        })
    });
}

fn bench_web_client_resolve_url(c: &mut Criterion) {
    let client = WebClientConfig::new("https://api.cenyslovensko.sk/")
        .build()
        .unwrap();

    c.bench_function("web_client_resolve_url", |b| {
        b.iter(|| client.resolve_url(black_box("version")).unwrap())
    });
}

criterion_group!(benches, bench_web_client_build, bench_web_client_resolve_url,);
criterion_main!(benches);
