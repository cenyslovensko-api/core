use derive_more::{Display, Error};
use miette::Diagnostic;

#[derive(Display, Error, Diagnostic, Debug, Clone, PartialEq, Eq)]
#[diagnostic(url("https://docs.rs/cenyslovensko_product_api"))]
pub enum ProductCategoryError {
    #[display("ProductCategory not found")]
    #[diagnostic(
        code(cenyslovensko::product_category::not_found),
        help(
            "The product endpoint returned 404 - check that the API base URI and product path are correct"
        )
    )]
    NotFound,

    #[display("ProductCategory response is invalid: {_0}")]
    #[diagnostic(
        code(cenyslovensko::product_category::invalid_response),
        help("The API returned a response that could not be parsed as a product object")
    )]
    InvalidResponse(#[error(not(source))] String),

    #[display("ProductCategory source is unavailable: {_0}")]
    #[diagnostic(
        code(cenyslovensko::product_category::unavailable),
        help("Check network connectivity and that the API base URI is reachable")
    )]
    Unavailable(#[error(not(source))] String),
}
