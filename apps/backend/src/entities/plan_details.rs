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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductsCreate {
    pub items: Vec<ProductItem>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanDetailsRead {
    pub product: ProductsRead,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlanDetailsCreate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<ProductsCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
}
