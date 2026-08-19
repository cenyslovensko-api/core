use crate::domain::product_category::ProductCategory;
use crate::domain::product_category_error::ProductCategoryError;
use crate::ports::product_category_source::ProductCategorySource;

pub struct GetProductCategoriesUseCase<TProductCategorySource>
where
    TProductCategorySource: ProductCategorySource,
{
    product_category_source: TProductCategorySource,
}

impl<TProductCategorySource> GetProductCategoriesUseCase<TProductCategorySource>
where
    TProductCategorySource: ProductCategorySource,
{
    pub fn new(product_category_source: TProductCategorySource) -> Self {
        Self { product_category_source }
    }

    pub async fn execute(&self) -> Result<Vec<ProductCategory>, ProductCategoryError> {
        self.product_category_source.get_product_categories().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::product_category::{ProductCategory, ProductSubcategory, ProductType};
    use std::sync::Mutex;

    struct FakeProductCategorySource {
        result: Mutex<Option<Result<Vec<ProductCategory>, ProductCategoryError>>>,
    }

    impl ProductCategorySource for FakeProductCategorySource {
        async fn get_product_categories(&self) -> Result<Vec<ProductCategory>, ProductCategoryError> {
            self.result
                .lock()
                .expect("fake source mutex should not be poisoned")
                .take()
                .expect("fake source result should be set")
        }
    }

    #[tokio::test]
    async fn returns_product_categories_from_source() {
        let use_case = GetProductCategoriesUseCase::new(FakeProductCategorySource {
            result: Mutex::new(Some(Ok(vec![ProductCategory {
                id: "cat_1".into(),
                category_name: "Food".into(),
                subcategories: vec![ProductSubcategory {
                    id: "sub_1".into(),
                    subcategory_name: "Bakery".into(),
                    types: vec![ProductType {
                        id: "type_1".into(),
                        type_name: "Bread".into(),
                    }],
                }],
            }]))),
        });

        let result = use_case.execute().await;

        let categories = result.expect("expected categories from source");
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
    async fn returns_error_from_source() {
        let use_case = GetProductCategoriesUseCase::new(FakeProductCategorySource {
            result: Mutex::new(Some(Err(ProductCategoryError::NotFound))),
        });

        let result = use_case.execute().await;

        assert!(matches!(result, Err(ProductCategoryError::NotFound)));
    }
}
