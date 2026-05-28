#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub brand: String,
    pub category: String,
}

impl Product {
    pub fn new(id: u32, name: &str, brand: &str, category: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            brand: brand.to_string(),
            category: category.to_string(),
        }
    }
}
