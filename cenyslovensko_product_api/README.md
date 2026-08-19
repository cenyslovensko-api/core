# CenySlovensko Product API

Rust API crate for retrieving CenySlovensko product data. It follows ports-and-adapters architecture and uses
`cenyslovensko_web_client` for HTTP configuration.

> [!NOTE]
> Intended to be used by in-process or local bindings (Python, Ruby, Node.js) through JSON-RPC calls.

> [!IMPORTANT]
> This project is not affiliated with or endorsed by CenySlovensko. It is an independent implementation.

## Examples

```rust
use cenyslovensko_product_api::{
    adapters::out::http::{
        product_category::HttpProductCategorySource,
        product_price::HttpProductPriceSource,
    },
    application::{
        get_current_day_product_prices_use_case::GetCurrentDayProductPricesUseCase,
        get_product_categories_use_case::GetProductCategoriesUseCase,
    },
    domain::product_price::{ProductPricesCurrentDayQuery, SortOrder},
};
use cenyslovensko_web_client::WebClientConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let web_client = WebClientConfig::new("https://api.cenyslovensko.sk/").build()?;
    let categories_source =
        HttpProductCategorySource::new(web_client.clone(), "api/product-categories");
    let categories_use_case = GetProductCategoriesUseCase::new(categories_source);
    let categories = categories_use_case.execute().await?;

    let prices_source = HttpProductPriceSource::new(web_client, "api/product-prices/current-day");
    let prices_use_case = GetCurrentDayProductPricesUseCase::new(prices_source);
    let query = ProductPricesCurrentDayQuery::builder()
        .branch_ids(["1061_50020188", "1015_50020188"])
        .order_by("unit_price")
        .sort_order(SortOrder::Asc)
        .only_in_my_branches(true)
        .category_id(2)
        .group_by_vendor(false)
        .page(0)
        .size(124)
        .build();
    let prices = prices_use_case.execute(query).await?;

    println!("Found {} categories", categories.len());
    println!("Found {} products", prices.count);
    Ok(())
}
```
