use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SortOrder {
    Asc,
    Desc,
}

impl SortOrder {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Asc => "asc",
            Self::Desc => "desc",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ProductPricesCurrentDayQuery {
    pub branch_ids: Vec<String>,
    pub order_by: Option<String>,
    pub sort_order: Option<SortOrder>,
    pub only_in_my_branches: Option<bool>,
    pub category_id: Option<u64>,
    pub group_by_vendor: Option<bool>,
    pub page: Option<u64>,
    pub size: Option<u64>,
}

impl ProductPricesCurrentDayQuery {
    pub fn builder() -> ProductPricesCurrentDayQueryBuilder {
        ProductPricesCurrentDayQueryBuilder::new()
    }

    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        if !self.branch_ids.is_empty() {
            params.push(("branchIds".to_string(), self.branch_ids.join(",")));
        }
        if let Some(order_by) = &self.order_by {
            params.push(("orderBy".to_string(), order_by.clone()));
        }
        if let Some(sort_order) = &self.sort_order {
            params.push(("sortOrder".to_string(), sort_order.as_str().to_string()));
        }
        if let Some(only_in_my_branches) = self.only_in_my_branches {
            params.push((
                "onlyInMyBranches".to_string(),
                only_in_my_branches.to_string(),
            ));
        }
        if let Some(category_id) = self.category_id {
            params.push(("categoryId".to_string(), category_id.to_string()));
        }
        if let Some(group_by_vendor) = self.group_by_vendor {
            params.push(("groupByVendor".to_string(), group_by_vendor.to_string()));
        }
        if let Some(page) = self.page {
            params.push(("page".to_string(), page.to_string()));
        }
        if let Some(size) = self.size {
            params.push(("size".to_string(), size.to_string()));
        }

        params
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ProductPricesCurrentDayQueryBuilder {
    query: ProductPricesCurrentDayQuery,
}

impl ProductPricesCurrentDayQueryBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn branch_ids<I, S>(mut self, branch_ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.query.branch_ids = branch_ids.into_iter().map(Into::into).collect();
        self
    }

    pub fn add_branch_id(mut self, branch_id: impl Into<String>) -> Self {
        self.query.branch_ids.push(branch_id.into());
        self
    }

    pub fn order_by(mut self, order_by: impl Into<String>) -> Self {
        self.query.order_by = Some(order_by.into());
        self
    }

    pub fn sort_order(mut self, sort_order: SortOrder) -> Self {
        self.query.sort_order = Some(sort_order);
        self
    }

    pub fn only_in_my_branches(mut self, only_in_my_branches: bool) -> Self {
        self.query.only_in_my_branches = Some(only_in_my_branches);
        self
    }

    pub fn category_id(mut self, category_id: u64) -> Self {
        self.query.category_id = Some(category_id);
        self
    }

    pub fn group_by_vendor(mut self, group_by_vendor: bool) -> Self {
        self.query.group_by_vendor = Some(group_by_vendor);
        self
    }

    pub fn page(mut self, page: u64) -> Self {
        self.query.page = Some(page);
        self
    }

    pub fn size(mut self, size: u64) -> Self {
        self.query.size = Some(size);
        self
    }

    pub fn build(self) -> ProductPricesCurrentDayQuery {
        self.query
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentDayProductPricesPage {
    pub page: u64,
    pub size: u64,
    pub count: u64,
    pub content: Vec<CurrentDayProductPriceItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentDayProductPriceItem {
    pub product_key: String,
    pub ean: String,
    pub internal_id: String,
    pub company_id: String,
    pub report_date: String,
    pub product_details: ProductDetails,
    #[serde(default)]
    pub prices: Vec<ProductBranchPrice>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductDetails {
    pub product_type: Option<String>,
    pub product_name: String,
    pub product_description: Option<String>,
    pub unit: Option<String>,
    pub package_size: Option<f64>,
    #[serde(default)]
    pub quality_standard: Vec<String>,
    pub picture: Option<String>,
    #[serde(default)]
    pub manufacturers: Vec<ProductCompany>,
    #[serde(default)]
    pub distributors: Vec<ProductCompany>,
    pub country_specifications: CountrySpecifications,
    pub product_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductCompany {
    pub name: String,
    #[serde(default)]
    pub country_codes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CountrySpecifications {
    #[serde(default)]
    pub breeding: Vec<String>,
    #[serde(default)]
    pub slaughter: Vec<String>,
    #[serde(default)]
    pub origin: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductBranchPrice {
    pub branch_id: String,
    pub price: Option<f64>,
    pub price_wo_tax: Option<f64>,
    pub tax_perc: Option<f64>,
    pub unit_price: Option<f64>,
    pub promo_price: Option<f64>,
    pub promo_price_wo_tax: Option<f64>,
    pub promo_from: Option<String>,
    pub promo_to: Option<String>,
    pub discount_percent: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_query_params_with_fluent_builder() {
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

        let params = query.to_query_params();

        assert_eq!(
            params,
            vec![
                ("branchIds".into(), "1061_50020188,1015_50020188".into()),
                ("orderBy".into(), "unit_price".into()),
                ("sortOrder".into(), "asc".into()),
                ("onlyInMyBranches".into(), "true".into()),
                ("categoryId".into(), "2".into()),
                ("groupByVendor".into(), "false".into()),
                ("page".into(), "0".into()),
                ("size".into(), "124".into())
            ]
        );
    }
}
