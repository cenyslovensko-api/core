use derive_more::Display;

#[derive(Display, Debug, Clone, PartialEq, Eq)]
pub enum VersionError {
    #[display("Version not found")]
    NotFound,
    #[display("Version response is invalid: {_0}")]
    InvalidResponse(String),
    #[display("Version source is unavailable: {_0}")]
    Unavailable(String),
}
