use crate::ports::ProductPricesGateway;
use cenyslovensko_api::product::adapters::out::http::product_price::HttpProductPriceSource;
use cenyslovensko_api::product::application::get_current_day_product_prices_use_case::GetCurrentDayProductPricesUseCase;
use cenyslovensko_api::product::domain::product_price::{
    CurrentDayProductPricesPage, ProductPricesCurrentDayQuery,
};
use cenyslovensko_api::product::domain::product_price_error::ProductPriceError;
use cenyslovensko_api::web_client::WebClient;

pub struct ProductPricesApiGateway {
    use_case: GetCurrentDayProductPricesUseCase<HttpProductPriceSource>,
}

impl ProductPricesApiGateway {
    pub fn new(web_client: WebClient, product_prices_current_day_path: impl Into<String>) -> Self {
        Self {
            use_case: GetCurrentDayProductPricesUseCase::new(HttpProductPriceSource::new(
                web_client,
                product_prices_current_day_path,
            )),
        }
    }
}

impl ProductPricesGateway for ProductPricesApiGateway {
    async fn get_current_day_product_prices(
        &self,
        query: ProductPricesCurrentDayQuery,
    ) -> Result<CurrentDayProductPricesPage, ProductPriceError> {
        self.use_case.execute(query).await
    }
}
