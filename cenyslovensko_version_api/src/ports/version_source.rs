use crate::domain::{Version, VersionError};
use std::future::Future;

pub trait VersionSource {
    fn get_version(&self) -> impl Future<Output = Result<Version, VersionError>> + Send;
}
