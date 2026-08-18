use derive_more::{Display, Error};
use miette::Diagnostic;

#[derive(Display, Error, Diagnostic, Debug, Clone, PartialEq, Eq)]
#[diagnostic(url("https://docs.rs/cenyslovensko_version_api"))]
pub enum VersionError {
    #[display("Version not found")]
    #[diagnostic(
        code(cenyslovensko::version::not_found),
        help(
            "The version endpoint returned 404 - check that the API base URI and version path are correct"
        )
    )]
    NotFound,

    #[display("Version response is invalid: {_0}")]
    #[diagnostic(
        code(cenyslovensko::version::invalid_response),
        help("The API returned a response that could not be parsed as a version object")
    )]
    InvalidResponse(#[error(not(source))] String),

    #[display("Version source is unavailable: {_0}")]
    #[diagnostic(
        code(cenyslovensko::version::unavailable),
        help("Check network connectivity and that the API base URI is reachable")
    )]
    Unavailable(#[error(not(source))] String),
}
