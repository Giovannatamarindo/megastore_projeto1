use std::collections::{HashMap, HashSet};

use crate::product::Product;

#[derive(Debug, Default)]
pub struct SearchEngine {
    products: HashMap<u32, Product>,
    name_index: HashMap<String, HashSet<u32>>,
    brand_index: HashMap<String, HashSet<u32>>,
    category_index: HashMap<String, HashSet<u32>>,
}

impl SearchEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_product(&mut self, product: Product) {
        let product_id = product.id;

        Self::index_text(&mut self.name_index, &product.name, product_id);
        Self::index_text(&mut self.brand_index, &product.brand, product_id);
        Self::index_text(&mut self.category_index, &product.category, product_id);

        self.products.insert(product_id, product);
    }

    pub fn search_by_name(&self, term: &str) -> Vec<Product> {
        self.search_in_index(&self.name_index, term)
    }

    pub fn search_by_brand(&self, term: &str) -> Vec<Product> {
        self.search_in_index(&self.brand_index, term)
    }

    pub fn search_by_category(&self, term: &str) -> Vec<Product> {
        self.search_in_index(&self.category_index, term)
    }

    pub fn search_all(&self, term: &str) -> Vec<Product> {
        let mut ids = HashSet::new();
        let normalized = Self::normalize(term);

        for index in [&self.name_index, &self.brand_index, &self.category_index] {
            if let Some(found_ids) = index.get(&normalized) {
                ids.extend(found_ids);
            }
        }

        self.products_from_ids(&ids)
    }

    fn index_text(index: &mut HashMap<String, HashSet<u32>>, text: &str, product_id: u32) {
        for word in text.split_whitespace() {
            let normalized = Self::normalize(word);
            index.entry(normalized).or_default().insert(product_id);
        }

        let full_text = Self::normalize(text);
        index.entry(full_text).or_default().insert(product_id);
    }

    fn search_in_index(&self, index: &HashMap<String, HashSet<u32>>, term: &str) -> Vec<Product> {
        let normalized = Self::normalize(term);

        match index.get(&normalized) {
            Some(ids) => self.products_from_ids(ids),
            None => Vec::new(),
        }
    }

    fn products_from_ids(&self, ids: &HashSet<u32>) -> Vec<Product> {
        let mut results: Vec<Product> = ids
            .iter()
            .filter_map(|id| self.products.get(id).cloned())
            .collect();

        results.sort_by_key(|product| product.id);
        results
    }

    fn normalize(text: &str) -> String {
        text.trim().to_lowercase()
    }
}
