use std::future::Future;

pub trait VersionGateway {
    fn get_version(&self) -> impl Future<Output = Result<String, String>>;
}
