use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductOption {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductItem {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[serde(default)]
    pub options: Vec<ProductOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductsRead {
    pub items: Vec<ProductItem>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductsUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ProductItem>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanDetailsRead {
    pub products: ProductsRead,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanDetailsUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub products: Option<ProductsUpdate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
}
