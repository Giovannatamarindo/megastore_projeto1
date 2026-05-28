use megastore_projeto2::product::Product;
use megastore_projeto2::search::SearchEngine;

fn create_engine() -> SearchEngine {
    let mut engine = SearchEngine::new();

    engine.add_product(Product::new(1, "Notebook Inspiron", "Dell", "Eletronicos"));
    engine.add_product(Product::new(2, "Smartphone Galaxy", "Samsung", "Eletronicos"));
    engine.add_product(Product::new(3, "Cafeteira Premium", "Philco", "Eletrodomesticos"));

    engine
}

#[test]
fn search_by_name_returns_product() {
    let engine = create_engine();
    let results = engine.search_by_name("Notebook");

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].brand, "Dell");
}

#[test]
fn search_by_brand_returns_product() {
    let engine = create_engine();
    let results = engine.search_by_brand("Samsung");

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Smartphone Galaxy");
}

#[test]
fn search_by_category_returns_multiple_products() {
    let engine = create_engine();
    let results = engine.search_by_category("Eletronicos");

    assert_eq!(results.len(), 2);
}

#[test]
fn search_all_finds_by_brand() {
    let engine = create_engine();
    let results = engine.search_all("Dell");

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Notebook Inspiron");
}

#[test]
fn search_without_results_returns_empty_vector() {
    let engine = create_engine();
    let results = engine.search_all("ProdutoInexistente");

    assert!(results.is_empty());
}
