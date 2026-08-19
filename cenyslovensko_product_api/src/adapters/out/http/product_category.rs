use crate::domain::product_category::{ProductCategory, ProductSubcategory, ProductType};
use crate::domain::product_category_error::ProductCategoryError;
use crate::ports::product_category_source::ProductCategorySource;
use cenyslovensko_web_client::WebClient;
use derive_more::Display;
use serde::Deserialize;

pub struct HttpProductCategorySource {
    product_category_url: String,
    web_client: WebClient,
}

impl HttpProductCategorySource {
    pub fn new(web_client: WebClient, product_category_url: impl Into<String>) -> Self {
        Self {
            product_category_url: product_category_url.into(),
            web_client,
        }
    }
}

#[derive(Display, Debug, Clone, Eq, PartialEq, Deserialize)]
#[display(
    "ProductCategoryResponse {{ id: {}, category_name: {}, subcategories: {:?} }}",
    id,
    category_name,
    subcategories
)]
#[serde(rename_all = "camelCase")]
pub struct ProductCategoryResponse {
    pub id: String,
    pub category_name: String,
    pub subcategories: Vec<ProductSubcategoryResponse>,
}

#[derive(Display, Debug, Clone, Eq, PartialEq, Deserialize)]
#[display(
    "ProductSubcategoryResponse {{ id: {}, subcategory_name: {}, types: {:?} }}",
    id,
    subcategory_name,
    types
)]
#[serde(rename_all = "camelCase")]
pub struct ProductSubcategoryResponse {
    pub id: String,
    pub subcategory_name: String,
    pub types: Vec<ProductTypeResponse>,
}

#[derive(Display, Debug, Clone, Eq, PartialEq, Deserialize)]
#[display("ProductTypeResponse {{ id: {}, type_name: {} }}", id, type_name)]
#[serde(rename_all = "camelCase")]
pub struct ProductTypeResponse {
    pub id: String,
    pub type_name: String,
}

impl From<ProductCategoryResponse> for ProductCategory {
    fn from(response: ProductCategoryResponse) -> Self {
        ProductCategory {
            id: response.id,
            category_name: response.category_name,
            subcategories: response
                .subcategories
                .into_iter()
                .map(ProductSubcategory::from)
                .collect(),
        }
    }
}

impl From<ProductSubcategoryResponse> for ProductSubcategory {
    fn from(response: ProductSubcategoryResponse) -> Self {
        ProductSubcategory {
            id: response.id,
            subcategory_name: response.subcategory_name,
            types: response.types.into_iter().map(ProductType::from).collect(),
        }
    }
}

impl From<ProductTypeResponse> for ProductType {
    fn from(response: ProductTypeResponse) -> Self {
        ProductType {
            id: response.id,
            type_name: response.type_name,
        }
    }
}

impl ProductCategorySource for HttpProductCategorySource {
    async fn get_product_categories(&self) -> Result<Vec<ProductCategory>, ProductCategoryError> {
        let endpoint = self
            .web_client
            .resolve_url(&self.product_category_url)
            .map_err(|error| ProductCategoryError::Unavailable(error.to_string()))?;

        let response = self
            .web_client
            .client()
            .get(endpoint)
            .send()
            .await
            .map_err(|error| ProductCategoryError::Unavailable(error.to_string()))?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(ProductCategoryError::NotFound);
        }

        let response = response
            .error_for_status()
            .map_err(|error| ProductCategoryError::Unavailable(error.to_string()))?;

        let response: Vec<ProductCategoryResponse> = response
            .json()
            .await
            .map_err(|error| ProductCategoryError::InvalidResponse(error.to_string()))?;

        Ok(response.into_iter().map(ProductCategory::from).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cenyslovensko_web_client::WebClientConfig;
    use httpmock::Method::GET;
    use httpmock::MockServer;

    #[tokio::test]
    async fn returns_product_categories_from_valid_json_response() {
        let server = MockServer::start();
        let _mock = server.mock(|when, then| {
            when.method(GET).path("/product-category");
            then.status(200)
                .header("content-type", "application/json")
                .body(
                    // language=json
                    r#"[
                    {
                        "id":"cat_1",
                        "categoryName":"Food",
                        "subcategories":[
                            {
                                "id":"sub_1",
                                "subcategoryName":"Bakery",
                                "types":[
                                    {"id":"type_1","typeName":"Bread"}
                                ]
                            }
                        ]
                    }
                ]"#,
                );
        });

        let web_client = WebClientConfig::new(server.base_url())
            .build()
            .expect("web client should build");
        let source = HttpProductCategorySource {
            product_category_url: "product-category".to_string(),
            web_client,
        };

        let result = source.get_product_categories().await;

        let categories = result.expect("expected successful product category response");
        assert_eq!(categories.len(), 1);
        assert_eq!(categories[0].id, "cat_1");
        assert_eq!(categories[0].category_name, "Food");
        assert_eq!(categories[0].subcategories.len(), 1);
        assert_eq!(categories[0].subcategories[0].id, "sub_1");
        assert_eq!(categories[0].subcategories[0].subcategory_name, "Bakery");
        assert_eq!(categories[0].subcategories[0].types.len(), 1);
        assert_eq!(categories[0].subcategories[0].types[0].id, "type_1");
        assert_eq!(categories[0].subcategories[0].types[0].type_name, "Bread");
    }

    #[tokio::test]
    async fn returns_not_found_for_404_response() {
        let server = MockServer::start();
        let _mock = server.mock(|when, then| {
            when.method(GET).path("/product-category");
            then.status(404);
        });

        let web_client = WebClientConfig::new(server.base_url())
            .build()
            .expect("web client should build");
        let source = HttpProductCategorySource {
            product_category_url: "product-category".to_string(),
            web_client,
        };

        let result = source.get_product_categories().await;

        assert!(matches!(result, Err(ProductCategoryError::NotFound)));
    }

    #[tokio::test]
    async fn returns_invalid_response_for_non_json_body() {
        let server = MockServer::start();
        let _mock = server.mock(|when, then| {
            when.method(GET).path("/product-category");
            then.status(200)
                .header("content-type", "application/json")
                .body(r#"{"missingProductCategories":"value"}"#);
        });

        let web_client = WebClientConfig::new(server.base_url())
            .build()
            .expect("web client should build");
        let source = HttpProductCategorySource {
            product_category_url: "product-category".to_string(),
            web_client,
        };

        let result = source.get_product_categories().await;

        assert!(matches!(
            result,
            Err(ProductCategoryError::InvalidResponse(_))
        ));
    }

    #[tokio::test]
    async fn returns_unavailable_for_server_errors() {
        let server = MockServer::start();
        let _mock = server.mock(|when, then| {
            when.method(GET).path("/product-category");
            then.status(500);
        });

        let web_client = WebClientConfig::new(server.base_url())
            .build()
            .expect("web client should build");
        let source = HttpProductCategorySource {
            product_category_url: "product-category".to_string(),
            web_client,
        };

        let result = source.get_product_categories().await;

        assert!(matches!(result, Err(ProductCategoryError::Unavailable(_))));
    }
}
