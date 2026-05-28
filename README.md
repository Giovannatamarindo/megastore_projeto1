# Sistema de Busca Otimizado para Catalogo de Produtos - MegaStore

## Descricao do projeto

Este projeto implementa um sistema de busca otimizado para o catalogo de produtos da MegaStore. A solucao utiliza a linguagem Rust e estruturas de dados baseadas em tabelas hash para indexar produtos por nome, marca e categoria, permitindo consultas mais rapidas e organizadas.

O objetivo e melhorar a experiencia de busca em um e-commerce com grande volume de produtos, reduzindo lentidao e aumentando a precisao dos resultados.

## Tecnologias utilizadas

- Rust 2021 Edition
- Cargo
- HashMap e HashSet da biblioteca padrao do Rust
- Testes com `cargo test`

## Estrutura do projeto

```text
megastore_projeto2/
├── Cargo.toml
├── README.md
├── DOCUMENTACAO_MEGASTORE.pdf
├── src/
│   ├── lib.rs
│   ├── main.rs
│   ├── product.rs
│   └── search.rs
└── tests/
    └── search_tests.rs
```

## Como executar o sistema

1. Instale o Rust pelo site oficial: https://www.rust-lang.org/tools/install
2. Abra o terminal na pasta do projeto.
3. Compile o projeto:

```bash
cargo build
```

4. Execute o sistema:

```bash
cargo run
```

O programa exibira exemplos de buscas por categoria e por termo geral.

## Como executar os testes

No terminal, dentro da pasta do projeto, execute:

```bash
cargo test
```

Os testes verificam busca por nome, marca, categoria e comportamento quando nenhum produto e encontrado.

## Exemplos de uso

Busca por categoria:

```rust
let results = engine.search_by_category("eletronicos");
```

Busca por marca:

```rust
let results = engine.search_by_brand("Samsung");
```

Busca geral:

```rust
let results = engine.search_all("Dell");
```

## Arquitetura do sistema

O sistema foi dividido em modulos:

- `product.rs`: define a estrutura `Product`, representando os produtos do catalogo.
- `search.rs`: define a estrutura `SearchEngine`, responsavel pela indexacao e busca.
- `lib.rs`: exporta os modulos principais para uso nos testes e no programa.
- `main.rs`: apresenta um exemplo pratico de uso do sistema.
- `tests/search_tests.rs`: contem testes de integracao.

## Algoritmos e estruturas de dados utilizados

A principal estrutura de dados utilizada e a tabela hash, por meio de `HashMap`. O sistema cria indices separados para:

- nome do produto;
- marca;
- categoria.

Cada indice associa uma palavra-chave a um conjunto de IDs de produtos, usando `HashSet`. Assim, a busca nao precisa percorrer todo o catalogo a cada consulta. Em vez disso, acessa diretamente a chave pesquisada no indice hash.

Essa estrategia melhora o desempenho, pois buscas em `HashMap` possuem tempo medio proximo de O(1).

## Consideracoes sobre desempenho e escalabilidade

A solucao foi pensada para lidar com um catalogo em crescimento. Como os produtos sao indexados em tabelas hash, a busca tende a ser rapida mesmo com muitos itens cadastrados.

Para um ambiente real de grande escala, seria possivel evoluir a solucao com:

- persistencia em banco de dados;
- cache distribuido;
- normalizacao de acentos;
- ranking de relevancia;
- busca aproximada para erros de digitacao;
- integracao com APIs da plataforma de e-commerce.

## Metricas de avaliacao

Algumas metricas que podem ser usadas para avaliar o sistema sao:

- tempo medio de resposta das buscas;
- quantidade de produtos indexados;
- quantidade de resultados relevantes retornados;
- uso de memoria;
- cobertura dos testes automatizados.

## Contribuicoes

Como este projeto e academico, contribuicoes podem ser feitas por meio de melhorias no codigo, aumento da cobertura de testes, melhoria da documentacao e inclusao de novas formas de busca.

## Licenca

Este projeto utiliza a licenca MIT para fins academicos.
