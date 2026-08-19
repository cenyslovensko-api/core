use crate::domain::product_price::{CurrentDayProductPricesPage, ProductPricesCurrentDayQuery};
use crate::domain::product_price_error::ProductPriceError;
use crate::ports::product_price_source::ProductPriceSource;
use cenyslovensko_web_client::WebClient;

pub struct HttpProductPriceSource {
    web_client: WebClient,
    product_prices_current_day_path: String,
}

impl HttpProductPriceSource {
    pub fn new(web_client: WebClient, product_prices_current_day_path: impl Into<String>) -> Self {
        Self {
            web_client,
            product_prices_current_day_path: product_prices_current_day_path.into(),
        }
    }
}

impl ProductPriceSource for HttpProductPriceSource {
    async fn get_current_day_product_prices(
        &self,
        query: ProductPricesCurrentDayQuery,
    ) -> Result<CurrentDayProductPricesPage, ProductPriceError> {
        if query.branch_ids.is_empty() {
            return Err(ProductPriceError::InvalidQuery(
                "branch_ids cannot be empty".to_string(),
            ));
        }

        let mut endpoint = self
            .web_client
            .resolve_url(&self.product_prices_current_day_path)
            .map_err(|error| ProductPriceError::Unavailable(error.to_string()))?;
        {
            let mut query_pairs = endpoint.query_pairs_mut();
            for (key, value) in query.to_query_params() {
                query_pairs.append_pair(&key, &value);
            }
        }

        let response = self
            .web_client
            .client()
            .get(endpoint)
            .send()
            .await
            .map_err(|error| ProductPriceError::Unavailable(error.to_string()))?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(ProductPriceError::NotFound);
        }

        let response = response
            .error_for_status()
            .map_err(|error| ProductPriceError::Unavailable(error.to_string()))?;

        response
            .json::<CurrentDayProductPricesPage>()
            .await
            .map_err(|error| ProductPriceError::InvalidResponse(error.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::product_price::SortOrder;
    use cenyslovensko_web_client::WebClientConfig;
    use httpmock::Method::GET;
    use httpmock::MockServer;

    #[tokio::test]
    async fn returns_product_prices_from_valid_json_response() {
        let server = MockServer::start();
        let _mock = server.mock(|when, then| {
            when.method(GET)
                .path("/product-prices/current-day")
                .query_param("branchIds", "1061_50020188,1015_50020188")
                .query_param("orderBy", "unit_price")
                .query_param("sortOrder", "asc")
                .query_param("onlyInMyBranches", "true")
                .query_param("categoryId", "2")
                .query_param("groupByVendor", "false")
                .query_param("page", "0")
                .query_param("size", "124");
            then.status(200)
                .header("content-type", "application/json")
                .body(
                    // language=json
                    r#"{
                      "page":0,
                      "size":124,
                      "count":1,
                      "content":[
                        {
                          "productKey":"e:8585002520203_50020188",
                          "ean":"8585002520203",
                          "internalId":"CK99996504",
                          "companyId":"50020188",
                          "reportDate":"2026-08-19T00:00:00.000+00:00",
                          "productDetails":{
                            "productType":"bs",
                            "productName":"Smot.na šľahanie 33% 180ml RAJO",
                            "productDescription":"Smot.na šľahanie 33% 180ml RAJO",
                            "unit":"l",
                            "packageSize":0.18,
                            "qualityStandard":[],
                            "picture":"CK99996504.jpg",
                            "manufacturers":[],
                            "distributors":[{"name":"MEGGLE Slovakia s. r. o.","countryCodes":["SVK"]}],
                            "countrySpecifications":{"breeding":[],"slaughter":[],"origin":["SVK"]},
                            "productUrl":null
                          },
                          "prices":[
                            {
                              "branchId":"1061_50020188",
                              "price":1.29,
                              "priceWoTax":1.08,
                              "taxPerc":19.0,
                              "unitPrice":7.167,
                              "promoPrice":null,
                              "promoPriceWoTax":null,
                              "promoFrom":null,
                              "promoTo":null,
                              "discountPercent":null
                            }
                          ]
                        }
                      ]
                    }"#,
                );
        });

        let web_client = WebClientConfig::new(server.base_url())
            .build()
            .expect("web client should build");
        let source = HttpProductPriceSource::new(web_client, "product-prices/current-day");
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

        let result = source.get_current_day_product_prices(query).await;

        let response = result.expect("expected successful response");
        assert_eq!(response.page, 0);
        assert_eq!(response.size, 124);
        assert_eq!(response.count, 1);
        assert_eq!(response.content[0].prices[0].branch_id, "1061_50020188");
    }

    #[tokio::test]
    async fn returns_invalid_query_for_missing_branch_ids() {
        let server = MockServer::start();
        let web_client = WebClientConfig::new(server.base_url())
            .build()
            .expect("web client should build");
        let source = HttpProductPriceSource::new(web_client, "product-prices/current-day");

        let result = source
            .get_current_day_product_prices(ProductPricesCurrentDayQuery::builder().build())
            .await;

        assert!(matches!(result, Err(ProductPriceError::InvalidQuery(_))));
    }

    #[tokio::test]
    async fn returns_not_found_for_404_response() {
        let server = MockServer::start();
        let _mock = server.mock(|when, then| {
            when.method(GET)
                .path("/product-prices/current-day")
                .query_param("branchIds", "1061_50020188");
            then.status(404);
        });

        let web_client = WebClientConfig::new(server.base_url())
            .build()
            .expect("web client should build");
        let source = HttpProductPriceSource::new(web_client, "product-prices/current-day");
        let query = ProductPricesCurrentDayQuery::builder()
            .add_branch_id("1061_50020188")
            .build();

        let result = source.get_current_day_product_prices(query).await;

        assert!(matches!(result, Err(ProductPriceError::NotFound)));
    }

    #[tokio::test]
    async fn returns_invalid_response_for_non_json_body() {
        let server = MockServer::start();
        let _mock = server.mock(|when, then| {
            when.method(GET)
                .path("/product-prices/current-day")
                .query_param("branchIds", "1061_50020188");
            then.status(200)
                .header("content-type", "application/json")
                .body(r#"{"missing_content":true}"#);
        });

        let web_client = WebClientConfig::new(server.base_url())
            .build()
            .expect("web client should build");
        let source = HttpProductPriceSource::new(web_client, "product-prices/current-day");
        let query = ProductPricesCurrentDayQuery::builder()
            .add_branch_id("1061_50020188")
            .build();

        let result = source.get_current_day_product_prices(query).await;

        assert!(matches!(result, Err(ProductPriceError::InvalidResponse(_))));
    }

    #[tokio::test]
    async fn returns_unavailable_for_server_errors() {
        let server = MockServer::start();
        let _mock = server.mock(|when, then| {
            when.method(GET)
                .path("/product-prices/current-day")
                .query_param("branchIds", "1061_50020188");
            then.status(500);
        });

        let web_client = WebClientConfig::new(server.base_url())
            .build()
            .expect("web client should build");
        let source = HttpProductPriceSource::new(web_client, "product-prices/current-day");
        let query = ProductPricesCurrentDayQuery::builder()
            .add_branch_id("1061_50020188")
            .build();

        let result = source.get_current_day_product_prices(query).await;

        assert!(matches!(result, Err(ProductPriceError::Unavailable(_))));
    }
}
