use crate::domain::{
    PRODUCT_PRICES_CURRENT_DAY_GET_METHOD, RpcRequest, RpcResponse, VENDOR_GET_METHOD,
    VERSION_GET_METHOD,
};
use crate::ports::{ProductPricesGateway, RpcRequestHandler, VendorGateway, VersionGateway};
use cenyslovensko_api::product::domain::product_price::{ProductPricesCurrentDayQuery, SortOrder};
use serde::Deserialize;
use serde_json::json;

#[doc(hidden)]
pub struct MissingVersionGateway;
#[doc(hidden)]
pub struct MissingVendorGateway;
#[doc(hidden)]
pub struct MissingProductPricesGateway;

pub struct RpcApplication<TVersionGateway, TVendorGateway, TProductPricesGateway> {
    version_gateway: TVersionGateway,
    vendor_gateway: TVendorGateway,
    product_prices_gateway: TProductPricesGateway,
}

pub struct RpcApplicationBuilder<
    TVersionGateway = MissingVersionGateway,
    TVendorGateway = MissingVendorGateway,
    TProductPricesGateway = MissingProductPricesGateway,
> {
    version_gateway: TVersionGateway,
    vendor_gateway: TVendorGateway,
    product_prices_gateway: TProductPricesGateway,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProductPricesCurrentDayParams {
    branch_ids: Vec<String>,
    order_by: Option<String>,
    sort_order: Option<RpcSortOrder>,
    only_in_my_branches: Option<bool>,
    category_id: Option<u64>,
    group_by_vendor: Option<bool>,
    page: Option<u64>,
    size: Option<u64>,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum RpcSortOrder {
    Asc,
    Desc,
}

impl RpcApplicationBuilder {
    pub fn new() -> Self {
        Self {
            version_gateway: MissingVersionGateway,
            vendor_gateway: MissingVendorGateway,
            product_prices_gateway: MissingProductPricesGateway,
        }
    }
}

impl Default for RpcApplicationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl<TVersionGateway, TVendorGateway, TProductPricesGateway>
    RpcApplicationBuilder<TVersionGateway, TVendorGateway, TProductPricesGateway>
{
    pub fn version_gateway<TNewVersionGateway: VersionGateway>(
        self,
        version_gateway: TNewVersionGateway,
    ) -> RpcApplicationBuilder<TNewVersionGateway, TVendorGateway, TProductPricesGateway> {
        RpcApplicationBuilder {
            version_gateway,
            vendor_gateway: self.vendor_gateway,
            product_prices_gateway: self.product_prices_gateway,
        }
    }

    pub fn vendor_gateway<TNewVendorGateway: VendorGateway>(
        self,
        vendor_gateway: TNewVendorGateway,
    ) -> RpcApplicationBuilder<TVersionGateway, TNewVendorGateway, TProductPricesGateway> {
        RpcApplicationBuilder {
            version_gateway: self.version_gateway,
            vendor_gateway,
            product_prices_gateway: self.product_prices_gateway,
        }
    }

    pub fn product_prices_gateway<TNewProductPricesGateway: ProductPricesGateway>(
        self,
        product_prices_gateway: TNewProductPricesGateway,
    ) -> RpcApplicationBuilder<TVersionGateway, TVendorGateway, TNewProductPricesGateway> {
        RpcApplicationBuilder {
            version_gateway: self.version_gateway,
            vendor_gateway: self.vendor_gateway,
            product_prices_gateway,
        }
    }
}

impl<TVersionGateway, TVendorGateway, TProductPricesGateway>
    RpcApplicationBuilder<TVersionGateway, TVendorGateway, TProductPricesGateway>
where
    TVersionGateway: VersionGateway,
    TVendorGateway: VendorGateway,
    TProductPricesGateway: ProductPricesGateway,
{
    pub fn build(self) -> RpcApplication<TVersionGateway, TVendorGateway, TProductPricesGateway> {
        RpcApplication {
            version_gateway: self.version_gateway,
            vendor_gateway: self.vendor_gateway,
            product_prices_gateway: self.product_prices_gateway,
        }
    }
}

impl<TVersionGateway, TVendorGateway, TProductPricesGateway>
    RpcApplication<TVersionGateway, TVendorGateway, TProductPricesGateway>
where
    TVersionGateway: VersionGateway,
    TVendorGateway: VendorGateway,
    TProductPricesGateway: ProductPricesGateway,
{
    pub fn new(
        version_gateway: TVersionGateway,
        vendor_gateway: TVendorGateway,
        product_prices_gateway: TProductPricesGateway,
    ) -> Self {
        RpcApplicationBuilder::new()
            .version_gateway(version_gateway)
            .vendor_gateway(vendor_gateway)
            .product_prices_gateway(product_prices_gateway)
            .build()
    }
}

impl<TVersionGateway, TVendorGateway, TProductPricesGateway> RpcRequestHandler
    for RpcApplication<TVersionGateway, TVendorGateway, TProductPricesGateway>
where
    TVersionGateway: VersionGateway,
    TVendorGateway: VendorGateway,
    TProductPricesGateway: ProductPricesGateway,
{
    async fn handle_request(&self, request: RpcRequest) -> RpcResponse {
        match request.method.as_str() {
            VERSION_GET_METHOD => match self.version_gateway.get_version().await {
                Ok(version) => RpcResponse::success(request.id, json!({ "version": version })),
                Err(error) => RpcResponse::internal_error(request.id, error),
            },
            VENDOR_GET_METHOD => match self.vendor_gateway.get_vendor().await {
                Ok(vendors) => RpcResponse::success(request.id, json!({ "vendors": vendors })),
                Err(error) => RpcResponse::internal_error(request.id, error.to_string()),
            },
            PRODUCT_PRICES_CURRENT_DAY_GET_METHOD => {
                let params_value = match request.params {
                    Some(params) => params,
                    None => {
                        return RpcResponse::invalid_params(
                            request.id,
                            "Missing params for product-prices.current-day.get",
                        );
                    }
                };
                let params: ProductPricesCurrentDayParams =
                    match serde_json::from_value(params_value) {
                        Ok(params) => params,
                        Err(error) => {
                            return RpcResponse::invalid_params(
                                request.id,
                                format!(
                                    "Invalid params for product-prices.current-day.get: {error}"
                                ),
                            );
                        }
                    };
                if params.branch_ids.is_empty() {
                    return RpcResponse::invalid_params(
                        request.id,
                        "branchIds must contain at least one branch id",
                    );
                }

                let mut query_builder =
                    ProductPricesCurrentDayQuery::builder().branch_ids(params.branch_ids);
                if let Some(order_by) = params.order_by {
                    query_builder = query_builder.order_by(order_by);
                }
                if let Some(sort_order) = params.sort_order {
                    let sort_order = match sort_order {
                        RpcSortOrder::Asc => SortOrder::Asc,
                        RpcSortOrder::Desc => SortOrder::Desc,
                    };
                    query_builder = query_builder.sort_order(sort_order);
                }
                if let Some(only_in_my_branches) = params.only_in_my_branches {
                    query_builder = query_builder.only_in_my_branches(only_in_my_branches);
                }
                if let Some(category_id) = params.category_id {
                    query_builder = query_builder.category_id(category_id);
                }
                if let Some(group_by_vendor) = params.group_by_vendor {
                    query_builder = query_builder.group_by_vendor(group_by_vendor);
                }
                if let Some(page) = params.page {
                    query_builder = query_builder.page(page);
                }
                if let Some(size) = params.size {
                    query_builder = query_builder.size(size);
                }
                match self
                    .product_prices_gateway
                    .get_current_day_product_prices(query_builder.build())
                    .await
                {
                    Ok(product_prices) => RpcResponse::success(
                        request.id,
                        json!({ "product_prices": product_prices }),
                    ),
                    Err(error) => RpcResponse::internal_error(request.id, error.to_string()),
                }
            }
            _ => RpcResponse::method_not_found(request.id),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::{ProductPricesGateway, VendorGateway, VersionGateway};
    use cenyslovensko_api::product::domain::product_price::{
        CountrySpecifications, CurrentDayProductPriceItem, CurrentDayProductPricesPage,
        ProductBranchPrice, ProductCompany, ProductDetails, ProductPricesCurrentDayQuery,
    };
    use cenyslovensko_api::product::domain::product_price_error::ProductPriceError;
    use cenyslovensko_api::vendor::domain::vendor::{Vendor, VendorAddress, VendorLocation};
    use cenyslovensko_api::vendor::domain::vendor_error::VendorError;
    use serde_json::Value;

    #[derive(Clone)]
    struct FakeVersionGateway {
        result: Result<String, String>,
    }

    impl VersionGateway for FakeVersionGateway {
        async fn get_version(&self) -> Result<String, String> {
            self.result.clone()
        }
    }

    #[derive(Clone)]
    struct FakeVendorGateway {
        result: Result<Vec<Vendor>, VendorError>,
    }

    impl VendorGateway for FakeVendorGateway {
        async fn get_vendor(&self) -> Result<Vec<Vendor>, VendorError> {
            self.result.clone()
        }
    }

    #[derive(Clone)]
    struct FakeProductPricesGateway {
        result: Result<CurrentDayProductPricesPage, ProductPriceError>,
    }

    impl ProductPricesGateway for FakeProductPricesGateway {
        async fn get_current_day_product_prices(
            &self,
            _query: ProductPricesCurrentDayQuery,
        ) -> Result<CurrentDayProductPricesPage, ProductPriceError> {
            self.result.clone()
        }
    }

    fn sample_product_prices_response() -> CurrentDayProductPricesPage {
        CurrentDayProductPricesPage {
            page: 0,
            size: 124,
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
        }
    }

    #[tokio::test]
    async fn returns_version_for_version_get_method() {
        let app = RpcApplicationBuilder::new()
            .version_gateway(FakeVersionGateway {
                result: Ok("0.1.370".into()),
            })
            .vendor_gateway(FakeVendorGateway { result: Ok(vec![]) })
            .product_prices_gateway(FakeProductPricesGateway {
                result: Ok(sample_product_prices_response()),
            })
            .build();
        let request = RpcRequest {
            id: Value::from(1),
            method: VERSION_GET_METHOD.into(),
            params: None,
        };

        let response = app.handle_request(request).await;

        assert_eq!(response.error, None);
        assert_eq!(response.result, Some(json!({ "version": "0.1.370" })));
    }

    #[tokio::test]
    async fn returns_vendors_for_vendor_get_method() {
        let app = RpcApplicationBuilder::new()
            .version_gateway(FakeVersionGateway {
                result: Ok("0.1.370".into()),
            })
            .vendor_gateway(FakeVendorGateway {
                result: Ok(vec![Vendor::new(
                    "branch_1".into(),
                    "Main Branch".into(),
                    VendorAddress::new("Bratislava".into(), "123".into()),
                    "company_1".into(),
                    VendorLocation::new(48.8566, 2.3522),
                )]),
            })
            .product_prices_gateway(FakeProductPricesGateway {
                result: Ok(sample_product_prices_response()),
            })
            .build();
        let request = RpcRequest {
            id: Value::from(1),
            method: VENDOR_GET_METHOD.into(),
            params: None,
        };

        let response = app.handle_request(request).await;

        assert_eq!(response.error, None);
        assert_eq!(
            response.result,
            Some(json!({
                "vendors": [{
                    "branch_id": "branch_1",
                    "branch_name": "Main Branch",
                    "address": {
                        "city": "Bratislava",
                        "street_number": "123"
                    },
                    "company_id": "company_1",
                    "location": {
                        "lat": 48.8566,
                        "lng": 2.3522
                    }
                }]
            }))
        );
    }

    #[tokio::test]
    async fn returns_product_prices_for_current_day_method() {
        let app = RpcApplicationBuilder::new()
            .version_gateway(FakeVersionGateway {
                result: Ok("0.1.370".into()),
            })
            .vendor_gateway(FakeVendorGateway { result: Ok(vec![]) })
            .product_prices_gateway(FakeProductPricesGateway {
                result: Ok(sample_product_prices_response()),
            })
            .build();
        let request = RpcRequest {
            id: Value::from(1),
            method: PRODUCT_PRICES_CURRENT_DAY_GET_METHOD.into(),
            params: Some(json!({
                "branchIds": ["1061_50020188", "1015_50020188"],
                "orderBy": "unit_price",
                "sortOrder": "asc",
                "onlyInMyBranches": true,
                "categoryId": 2,
                "groupByVendor": false,
                "page": 0,
                "size": 124
            })),
        };

        let response = app.handle_request(request).await;

        assert_eq!(response.error, None);
        assert_eq!(
            response.result,
            Some(json!({ "product_prices": sample_product_prices_response() }))
        );
    }

    #[tokio::test]
    async fn returns_invalid_params_for_missing_product_prices_params() {
        let app = RpcApplicationBuilder::new()
            .version_gateway(FakeVersionGateway {
                result: Ok("0.1.370".into()),
            })
            .vendor_gateway(FakeVendorGateway { result: Ok(vec![]) })
            .product_prices_gateway(FakeProductPricesGateway {
                result: Ok(sample_product_prices_response()),
            })
            .build();
        let request = RpcRequest {
            id: Value::from(1),
            method: PRODUCT_PRICES_CURRENT_DAY_GET_METHOD.into(),
            params: None,
        };

        let response = app.handle_request(request).await;

        assert!(response.result.is_none());
        let error = response.error.expect("error should be present");
        assert_eq!(error.code, -32602);
        assert!(error.message.contains("Missing params"));
    }

    #[tokio::test]
    async fn returns_internal_error_for_vendor_failure() {
        let app = RpcApplicationBuilder::new()
            .version_gateway(FakeVersionGateway {
                result: Ok("0.1.370".into()),
            })
            .vendor_gateway(FakeVendorGateway {
                result: Err(VendorError::Unavailable("vendor unavailable".into())),
            })
            .product_prices_gateway(FakeProductPricesGateway {
                result: Ok(sample_product_prices_response()),
            })
            .build();
        let request = RpcRequest {
            id: Value::from(1),
            method: VENDOR_GET_METHOD.into(),
            params: None,
        };

        let response = app.handle_request(request).await;

        assert!(response.result.is_none());
        assert_eq!(
            response.error.expect("error should be present").message,
            "Vendor source is unavailable: vendor unavailable"
        );
    }

    #[tokio::test]
    async fn returns_method_not_found_for_unknown_method() {
        let app = RpcApplicationBuilder::new()
            .version_gateway(FakeVersionGateway {
                result: Ok("0.1.370".into()),
            })
            .vendor_gateway(FakeVendorGateway { result: Ok(vec![]) })
            .product_prices_gateway(FakeProductPricesGateway {
                result: Ok(sample_product_prices_response()),
            })
            .build();
        let request = RpcRequest {
            id: Value::from(1),
            method: "unknown.method".into(),
            params: None,
        };

        let response = app.handle_request(request).await;

        assert!(response.result.is_none());
        assert_eq!(
            response.error.expect("error should be present").code,
            -32601
        );
    }

    #[tokio::test]
    async fn builder_constructs_application() {
        let app = RpcApplicationBuilder::new()
            .vendor_gateway(FakeVendorGateway { result: Ok(vec![]) })
            .product_prices_gateway(FakeProductPricesGateway {
                result: Ok(sample_product_prices_response()),
            })
            .version_gateway(FakeVersionGateway {
                result: Ok("0.1.370".into()),
            })
            .build();
        let request = RpcRequest {
            id: Value::from(1),
            method: VERSION_GET_METHOD.into(),
            params: None,
        };

        let response = app.handle_request(request).await;

        assert_eq!(response.result, Some(json!({ "version": "0.1.370" })));
    }
}
