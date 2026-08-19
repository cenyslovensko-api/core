use crate::domain::product_price::{CurrentDayProductPricesPage, ProductPricesCurrentDayQuery};
use crate::domain::product_price_error::ProductPriceError;

pub trait ProductPriceSource {
    fn get_current_day_product_prices(
        &self,
        query: ProductPricesCurrentDayQuery,
    ) -> impl Future<Output = Result<CurrentDayProductPricesPage, ProductPriceError>> + Send;
}
