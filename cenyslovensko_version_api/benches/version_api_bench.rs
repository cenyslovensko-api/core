use cenyslovensko_version_api::application::GetVersionUseCase;
use cenyslovensko_version_api::domain::{Version, VersionError};
use cenyslovensko_version_api::ports::VersionSource;
use criterion::{Criterion, criterion_group, criterion_main};
use std::future::Future;
use std::hint::black_box;

#[derive(Clone)]
struct FakeVersionSource {
    version: String,
}

impl VersionSource for FakeVersionSource {
    fn get_version(&self) -> impl Future<Output = Result<Version, VersionError>> + Send {
        let version = self.version.clone();
        async move { Ok(Version::new(version)) }
    }
}

fn bench_version_new(c: &mut Criterion) {
    c.bench_function("version_new", |b| {
        b.iter(|| Version::new(black_box("0.1.370").to_string()))
    });
}

fn bench_version_to_string(c: &mut Criterion) {
    let version = Version::new("0.1.370".into());

    c.bench_function("version_to_string", |b| {
        b.iter(|| black_box(&version).to_string())
    });
}

fn bench_get_version_use_case(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let source = FakeVersionSource {
        version: "0.1.370".into(),
    };
    let use_case = GetVersionUseCase::new(source);

    c.bench_function("get_version_use_case_execute", |b| {
        b.to_async(&rt).iter(|| use_case.execute())
    });
}

criterion_group!(
    benches,
    bench_version_new,
    bench_version_to_string,
    bench_get_version_use_case,
);
criterion_main!(benches);
