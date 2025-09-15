# Módulo 9: Cargo e Crates

## 🎯 Objetivos de Aprendizagem

- Dominar Cargo como gerenciador de projetos
- Criar e publicar crates
- Gerenciar dependências
- Implementar testes e documentação

## 📚 Conteúdo Teórico

### 9.1 Cargo - Gerenciador de Projetos

```mermaid
graph TD
    A[Cargo] --> B[Project Management]
    A --> C[Dependency Management]
    A --> D[Build System]
    A --> E[Testing]
    
    B --> F[cargo new]
    B --> G[cargo init]
    
    C --> H[Cargo.toml]
    C --> I[crates.io]
    
    D --> J[cargo build]
    D --> K[cargo run]
    
    E --> L[cargo test]
    E --> M[#[cfg(test)]]
    
    style A fill:#e1f5fe
    style B fill:#f3e5f5
    style C fill:#fff3e0
    style D fill:#e8f5e8
    style E fill:#fce4ec
```

### 9.2 Exemplo Prático: Biblioteca Personalizada

```toml
# Cargo.toml
[package]
name = "minha-biblioteca"
version = "0.1.0"
edition = "2021"
authors = ["Seu Nome <seu@email.com>"]
description = "Uma biblioteca útil para cálculos matemáticos"
license = "MIT"
repository = "https://github.com/seu-usuario/minha-biblioteca"
keywords = ["math", "calculations", "utilities"]
categories = ["mathematics", "development-tools"]

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "math_bench"
harness = false

[lib]
name = "minha_biblioteca"
path = "src/lib.rs"

[[bin]]
name = "calculadora"
path = "src/bin/calculadora.rs"
```

```rust
// src/lib.rs
//! # Minha Biblioteca
//! 
//! Uma biblioteca para cálculos matemáticos úteis.

use serde::{Deserialize, Serialize};

/// Representa um ponto 2D
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    /// Cria um novo ponto 2D
    pub fn new(x: f64, y: f64) -> Self {
        Point2D { x, y }
    }
    
    /// Calcula a distância até outro ponto
    pub fn distance_to(&self, other: &Point2D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
    
    /// Calcula o ponto médio entre este ponto e outro
    pub fn midpoint(&self, other: &Point2D) -> Point2D {
        Point2D {
            x: (self.x + other.x) / 2.0,
            y: (self.y + other.y) / 2.0,
        }
    }
}

/// Calcula a área de um triângulo
pub fn triangle_area(p1: &Point2D, p2: &Point2D, p3: &Point2D) -> f64 {
    let a = p1.distance_to(p2);
    let b = p2.distance_to(p3);
    let c = p3.distance_to(p1);
    let s = (a + b + c) / 2.0;
    (s * (s - a) * (s - b) * (s - c)).sqrt()
}

/// Calcula o fatorial de um número
pub fn factorial(n: u32) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n as u64 * factorial(n - 1),
    }
}

/// Calcula os números de Fibonacci
pub fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_distance() {
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(3.0, 4.0);
        assert_eq!(p1.distance_to(&p2), 5.0);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(10), 55);
    }
}
```

```rust
// src/bin/calculadora.rs
use minha_biblioteca::{Point2D, triangle_area, factorial, fibonacci};
use serde_json;

fn main() {
    println!("=== Calculadora Matemática ===");
    
    // Testando pontos
    let p1 = Point2D::new(0.0, 0.0);
    let p2 = Point2D::new(3.0, 4.0);
    let p3 = Point2D::new(0.0, 4.0);
    
    println!("Distância entre pontos: {:.2}", p1.distance_to(&p2));
    println!("Área do triângulo: {:.2}", triangle_area(&p1, &p2, &p3));
    
    // Testando funções matemáticas
    println!("Fatorial de 5: {}", factorial(5));
    println!("Fibonacci(10): {}", fibonacci(10));
    
    // Serialização
    let json = serde_json::to_string_pretty(&p1).unwrap();
    println!("Ponto serializado: {}", json);
}
```

## 🎯 Tutorial Prático: Publicação de Crate

### Passos para Publicar

1. **Configurar Cargo.toml**
```toml
[package]
name = "meu-crate"
version = "0.1.0"
edition = "2021"
authors = ["Seu Nome <seu@email.com>"]
description = "Descrição do seu crate"
license = "MIT"
repository = "https://github.com/seu-usuario/meu-crate"
```

2. **Adicionar Documentação**
```rust
//! # Meu Crate
//! 
//! Descrição detalhada do que seu crate faz.

/// Função que faz algo útil
/// 
/// # Exemplos
/// 
/// ```
/// use meu_crate::minha_funcao;
/// 
/// let resultado = minha_funcao(5);
/// assert_eq!(resultado, 25);
/// ```
pub fn minha_funcao(x: i32) -> i32 {
    x * x
}
```

3. **Executar Testes**
```bash
cargo test
cargo test --doc  # Testes de documentação
```

4. **Verificar Código**
```bash
cargo check
cargo clippy
cargo fmt
```

5. **Publicar**
```bash
cargo login  # Login no crates.io
cargo publish
```

### Estrutura de Projeto Completa

```
meu-crate/
├── Cargo.toml
├── README.md
├── LICENSE
├── src/
│   ├── lib.rs
│   ├── bin/
│   │   └── exemplo.rs
│   ├── examples/
│   │   └── exemplo_uso.rs
│   └── tests/
│       └── integration_test.rs
├── benches/
│   └── benchmark.rs
└── docs/
    └── README.md
```

## 🎯 Atividades Práticas

### Atividade 1: Biblioteca de Utilitários
Crie uma biblioteca com funções úteis e publique no crates.io.

### Atividade 2: Framework de Testes
Desenvolva um framework de testes personalizado.

### Atividade 3: Biblioteca de Algoritmos
Implemente uma biblioteca com algoritmos de ordenação e busca.

---

**Professor:** Jackson Sá  
**ETEC Bento Quirino - Campinas/SP**
