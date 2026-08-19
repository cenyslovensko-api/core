pub struct ProductCategory {
    pub id: String,
    pub category_name: String,
    pub subcategories: Vec<ProductSubcategory>,
}

pub struct ProductSubcategory {
    pub id: String,
    pub subcategory_name: String,
    pub types: Vec<ProductType>,
}

pub struct ProductType {
    pub id: String,
    pub type_name: String,
}
