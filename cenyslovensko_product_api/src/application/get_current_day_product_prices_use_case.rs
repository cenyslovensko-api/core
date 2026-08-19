use crate::domain::product_price::{CurrentDayProductPricesPage, ProductPricesCurrentDayQuery};
use crate::domain::product_price_error::ProductPriceError;
use crate::ports::product_price_source::ProductPriceSource;

pub struct GetCurrentDayProductPricesUseCase<TProductPriceSource>
where
    TProductPriceSource: ProductPriceSource,
{
    product_price_source: TProductPriceSource,
}

impl<TProductPriceSource> GetCurrentDayProductPricesUseCase<TProductPriceSource>
where
    TProductPriceSource: ProductPriceSource,
{
    pub fn new(product_price_source: TProductPriceSource) -> Self {
        Self {
            product_price_source,
        }
    }

    pub async fn execute(
        &self,
        query: ProductPricesCurrentDayQuery,
    ) -> Result<CurrentDayProductPricesPage, ProductPriceError> {
        self.product_price_source
            .get_current_day_product_prices(query)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::product_price::{
        CountrySpecifications, CurrentDayProductPriceItem, ProductBranchPrice, ProductCompany,
        ProductDetails,
    };
    use std::sync::Mutex;

    struct FakeProductPriceSource {
        result: Mutex<Option<Result<CurrentDayProductPricesPage, ProductPriceError>>>,
    }

    impl ProductPriceSource for FakeProductPriceSource {
        async fn get_current_day_product_prices(
            &self,
            _query: ProductPricesCurrentDayQuery,
        ) -> Result<CurrentDayProductPricesPage, ProductPriceError> {
            self.result
                .lock()
                .expect("fake source mutex should not be poisoned")
                .take()
                .expect("fake source result should be set")
        }
    }

    #[tokio::test]
    async fn returns_current_day_product_prices_from_source() {
        let use_case = GetCurrentDayProductPricesUseCase::new(FakeProductPriceSource {
            result: Mutex::new(Some(Ok(CurrentDayProductPricesPage {
                page: 0,
                size: 1,
                count: 1,
                content: vec![CurrentDayProductPriceItem {
                    product_key: "e:8585002520203_50020188".into(),
                    ean: "8585002520203".into(),
                    internal_id: "CK99996504".into(),
                    company_id: "50020188".into(),
                    report_date: "2026-08-19T00:00:00.000+00:00".into(),
                    product_details: ProductDetails {
                        product_type: Some("bs".into()),
                        product_name: "Smot.na šľahanie 33% 180ml RAJO".into(),
                        product_description: Some("Smot.na šľahanie 33% 180ml RAJO".into()),
                        unit: Some("l".into()),
                        package_size: Some(0.18),
                        quality_standard: vec![],
                        picture: Some("CK99996504.jpg".into()),
                        manufacturers: vec![],
                        distributors: vec![ProductCompany {
                            name: "MEGGLE Slovakia s. r. o.".into(),
                            country_codes: vec!["SVK".into()],
                        }],
                        country_specifications: CountrySpecifications {
                            breeding: vec![],
                            slaughter: vec![],
                            origin: vec!["SVK".into()],
                        },
                        product_url: None,
                    },
                    prices: vec![ProductBranchPrice {
                        branch_id: "8102_50020188".into(),
                        price: Some(1.29),
                        price_wo_tax: Some(1.08),
                        tax_perc: Some(19.0),
                        unit_price: Some(7.167),
                        promo_price: None,
                        promo_price_wo_tax: None,
                        promo_from: None,
                        promo_to: None,
                        discount_percent: None,
                    }],
                }],
            }))),
        });

        let result = use_case
            .execute(
                ProductPricesCurrentDayQuery::builder()
                    .add_branch_id("8102_50020188")
                    .build(),
            )
            .await;

        let response = result.expect("expected product prices from source");
        assert_eq!(response.count, 1);
        assert_eq!(response.content[0].product_key, "e:8585002520203_50020188");
        assert_eq!(response.content[0].prices[0].branch_id, "8102_50020188");
    }

    #[tokio::test]
    async fn returns_error_from_source() {
        let use_case = GetCurrentDayProductPricesUseCase::new(FakeProductPriceSource {
            result: Mutex::new(Some(Err(ProductPriceError::NotFound))),
        });

        let result = use_case
            .execute(
                ProductPricesCurrentDayQuery::builder()
                    .add_branch_id("8102_50020188")
                    .build(),
            )
            .await;

        assert!(matches!(result, Err(ProductPriceError::NotFound)));
    }
}
