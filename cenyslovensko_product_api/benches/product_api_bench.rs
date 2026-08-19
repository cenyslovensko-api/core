use cenyslovensko_product_api::adapters::out::http::product_price::HttpProductPriceSource;
use cenyslovensko_product_api::application::get_current_day_product_prices_use_case::GetCurrentDayProductPricesUseCase;
use cenyslovensko_product_api::domain::product_price::{
    CountrySpecifications, CurrentDayProductPriceItem, CurrentDayProductPricesPage,
    ProductBranchPrice, ProductCompany, ProductDetails, ProductPricesCurrentDayQuery, SortOrder,
};
use cenyslovensko_product_api::domain::product_price_error::ProductPriceError;
use cenyslovensko_product_api::ports::product_price_source::ProductPriceSource;
use cenyslovensko_web_client::WebClientConfig;
use criterion::{Criterion, criterion_group, criterion_main};
use httpmock::Method::GET;
use httpmock::MockServer;
use std::hint::black_box;
use tokio::runtime::Runtime;

#[derive(Clone)]
struct FakeProductPriceSource;

impl ProductPriceSource for FakeProductPriceSource {
    async fn get_current_day_product_prices(
        &self,
        _query: ProductPricesCurrentDayQuery,
    ) -> Result<CurrentDayProductPricesPage, ProductPriceError> {
        Ok(CurrentDayProductPricesPage {
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
                    branch_id: "1061_50020188".into(),
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
        })
    }
}

fn benchmark_query_builder_to_query_params(c: &mut Criterion) {
    c.bench_function("product_prices_query_builder_to_query_params", |b| {
        b.iter(|| {
            let query = ProductPricesCurrentDayQuery::builder()
                .branch_ids(["1061_50020188", "1015_50020188", "8102_50020188"])
                .order_by("unit_price")
                .sort_order(SortOrder::Asc)
                .only_in_my_branches(true)
                .category_id(2)
                .group_by_vendor(false)
                .page(0)
                .size(124)
                .build();
            black_box(query.to_query_params())
        })
    });
}

fn benchmark_get_current_day_product_prices_use_case(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let source = FakeProductPriceSource;
    let use_case = GetCurrentDayProductPricesUseCase::new(source);
    let query = ProductPricesCurrentDayQuery::builder()
        .branch_ids(["1061_50020188", "1015_50020188", "8102_50020188"])
        .order_by("unit_price")
        .sort_order(SortOrder::Asc)
        .only_in_my_branches(true)
        .category_id(2)
        .group_by_vendor(false)
        .page(0)
        .size(124)
        .build();

    c.bench_function("get_current_day_product_prices_use_case_execute", |b| {
        b.to_async(&rt)
            .iter(|| use_case.execute(black_box(query.clone())))
    });
}

fn benchmark_http_product_price_source(c: &mut Criterion) {
    let server = MockServer::start();
    let _mock = server.mock(|when, then| {
        when.method(GET)
            .path("/product-prices/current-day")
            .query_param("branchIds", "1061_50020188,1015_50020188,8102_50020188")
            .query_param("orderBy", "unit_price")
            .query_param("sortOrder", "asc")
            .query_param("onlyInMyBranches", "true")
            .query_param("categoryId", "2")
            .query_param("groupByVendor", "false")
            .query_param("page", "0")
            .query_param("size", "124");
        then.status(200).header("content-type", "application/json").body(
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

    let client = WebClientConfig::new(server.base_url()).build().unwrap();
    let source = HttpProductPriceSource::new(client, "product-prices/current-day");
    let query = ProductPricesCurrentDayQuery::builder()
        .branch_ids(["1061_50020188", "1015_50020188", "8102_50020188"])
        .order_by("unit_price")
        .sort_order(SortOrder::Asc)
        .only_in_my_branches(true)
        .category_id(2)
        .group_by_vendor(false)
        .page(0)
        .size(124)
        .build();
    let rt = Runtime::new().unwrap();

    c.bench_function(
        "http_product_price_source_get_current_day_product_prices",
        |b| {
            b.to_async(&rt)
                .iter(|| source.get_current_day_product_prices(black_box(query.clone())))
        },
    );
}

criterion_group!(
    benches,
    benchmark_query_builder_to_query_params,
    benchmark_get_current_day_product_prices_use_case,
    benchmark_http_product_price_source,
);

criterion_main!(benches);
