use derive_more::{Display, Error};
use miette::Diagnostic;

#[derive(Display, Error, Diagnostic, Debug, Clone, PartialEq, Eq)]
#[diagnostic(url("https://docs.rs/cenyslovensko_product_api"))]
pub enum ProductPriceError {
    #[display("Product prices not found")]
    #[diagnostic(
        code(cenyslovensko::product_price::not_found),
        help(
            "The product-prices endpoint returned 404 - check that the API base URI and product prices path are correct"
        )
    )]
    NotFound,

    #[display("Product prices query is invalid: {_0}")]
    #[diagnostic(
        code(cenyslovensko::product_price::invalid_query),
        help("Provide valid query parameters for current-day product prices")
    )]
    InvalidQuery(#[error(not(source))] String),

    #[display("Product prices response is invalid: {_0}")]
    #[diagnostic(
        code(cenyslovensko::product_price::invalid_response),
        help("The API returned a response that could not be parsed as current-day product prices")
    )]
    InvalidResponse(#[error(not(source))] String),

    #[display("Product prices source is unavailable: {_0}")]
    #[diagnostic(
        code(cenyslovensko::product_price::unavailable),
        help("Check network connectivity and that the API base URI is reachable")
    )]
    Unavailable(#[error(not(source))] String),
}
