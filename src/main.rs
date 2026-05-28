use megastore_projeto2::product::Product;
use megastore_projeto2::search::SearchEngine;

fn main() {
    let mut engine = SearchEngine::new();

    engine.add_product(Product::new(1, "Notebook Inspiron", "Dell", "Eletronicos"));
    engine.add_product(Product::new(2, "Smartphone Galaxy", "Samsung", "Eletronicos"));
    engine.add_product(Product::new(3, "Cafeteira Premium", "Philco", "Eletrodomesticos"));
    engine.add_product(Product::new(4, "Monitor UltraWide", "LG", "Eletronicos"));

    println!("Busca por categoria 'eletronicos':");
    for product in engine.search_by_category("eletronicos") {
        println!("{:?}", product);
    }

    println!("\nBusca geral por 'Dell':");
    for product in engine.search_all("Dell") {
        println!("{:?}", product);
    }
}
