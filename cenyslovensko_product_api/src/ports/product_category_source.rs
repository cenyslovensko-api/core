use crate::domain::product_category::ProductCategory;
use crate::domain::product_category_error::ProductCategoryError;

pub trait ProductCategorySource {
    fn get_product_categories(
        &self,
    ) -> impl Future<Output = Result<Vec<ProductCategory>, ProductCategoryError>> + Send;
}
