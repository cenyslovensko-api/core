use derive_more::{Display, Error};
use miette::Diagnostic;

#[derive(Display, Error, Diagnostic, Debug, Clone, PartialEq, Eq)]
#[diagnostic(url("https://docs.rs/cenyslovensko_vendor_api"))]
pub enum VendorError {
    #[display("Vendor not found")]
    #[diagnostic(
        code(cenyslovensko::vendor::not_found),
        help(
            "The vendor endpoint returned 404 - check that the API base URI and vendor path are correct"
        )
    )]
    NotFound,

    #[display("Vendor response is invalid: {_0}")]
    #[diagnostic(
        code(cenyslovensko::vendor::invalid_response),
        help("The API returned a response that could not be parsed as a vendor object")
    )]
    InvalidResponse(#[error(not(source))] String),

    #[display("Vendor source is unavailable: {_0}")]
    #[diagnostic(
        code(cenyslovensko::vendor::unavailable),
        help("Check network connectivity and that the API base URI is reachable")
    )]
    Unavailable(#[error(not(source))] String),
}
