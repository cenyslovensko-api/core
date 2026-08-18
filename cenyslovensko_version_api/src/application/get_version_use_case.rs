use crate::domain::{Version, VersionError};
use crate::ports::VersionSource;

pub struct GetVersionUseCase<TVersionSource>
where
    TVersionSource: VersionSource,
{
    version_source: TVersionSource,
}

impl<TVersionSource> GetVersionUseCase<TVersionSource>
where
    TVersionSource: VersionSource,
{
    pub fn new(version_source: TVersionSource) -> Self {
        Self { version_source }
    }

    pub async fn execute(&self) -> Result<Version, VersionError> {
        self.version_source.get_version().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::future::Future;

    #[derive(Clone)]
    struct FakeVersionSource {
        result: Result<Version, VersionError>,
    }

    impl VersionSource for FakeVersionSource {
        fn get_version(&self) -> impl Future<Output = Result<Version, VersionError>> + Send {
            let result = self.result.clone();
            async move { result }
        }
    }

    #[tokio::test]
    async fn returns_version_from_source() {
        let source = FakeVersionSource {
            result: Ok(Version::new("0.1.370".into())),
        };
        let use_case = GetVersionUseCase::new(source);

        let result = use_case.execute().await;

        assert_eq!(result, Ok(Version::new("0.1.370".into())));
    }

    #[tokio::test]
    async fn propagates_source_error() {
        let source = FakeVersionSource {
            result: Err(VersionError::NotFound),
        };
        let use_case = GetVersionUseCase::new(source);

        let result = use_case.execute().await;

        assert_eq!(result, Err(VersionError::NotFound));
    }
}
