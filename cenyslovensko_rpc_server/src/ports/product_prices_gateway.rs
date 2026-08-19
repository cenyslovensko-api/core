use cenyslovensko_api::product::domain::product_price::{
    CurrentDayProductPricesPage, ProductPricesCurrentDayQuery,
};
use cenyslovensko_api::product::domain::product_price_error::ProductPriceError;

pub trait ProductPricesGateway {
    fn get_current_day_product_prices(
        &self,
        query: ProductPricesCurrentDayQuery,
    ) -> impl Future<Output = Result<CurrentDayProductPricesPage, ProductPriceError>>;
}
