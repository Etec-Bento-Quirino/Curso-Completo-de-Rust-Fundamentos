# Módulo 10: Projetos Práticos

## 🎯 Objetivos de Aprendizagem

Ao final deste módulo, você será capaz de:

- ✅ Integrar todos os conceitos aprendidos
- ✅ Criar projetos completos e funcionais
- ✅ Implementar sistemas reais
- ✅ Desenvolver portfólio profissional

## 📋 **Pré-requisitos**

### **Obrigatórios**
- ✅ **Conhecimento básico de programação** - Variáveis, funções, estruturas de controle
- ✅ **Rust (versão 1.70 ou superior)** - [Instalar Rust](https://rustup.rs/)
- ✅ **Cargo (gerenciador de pacotes)** - Instalado automaticamente com Rust
- ✅ **Editor de código** - [VS Code com rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) recomendado
- ✅ **Git** - Para controle de versão
- ✅ **Conclusão do Módulo 9** - [Cargo e Crates](../modulo-09-cargo-crates/README.md)

### **Recomendados**
- ✅ **Familiaridade com linha de comando** - Terminal/CMD básico
- ✅ **Conceitos básicos de programação funcional** - Funções, imutabilidade
- ✅ **Experiência com outras linguagens** - C/C++, Python, JavaScript, etc.
- ✅ **Conceitos de sistemas operacionais** - Memória, processos, threads

### **Recursos de Preparação**
- [**Rustlings**](https://github.com/rust-lang/rustlings) - Exercícios interativos para iniciantes
- [**The Rust Book**](https://doc.rust-lang.org/book/) - Documentação oficial completa
- [**Rust by Example**](https://doc.rust-lang.org/rust-by-example/) - Exemplos práticos
- [**Rust Playground**](https://play.rust-lang.org/) - Ambiente online para experimentar

## 📚 **Recursos de Aprendizado**

### **Documentação Oficial**
- [**The Rust Book - Final Project**](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
- [**Rust by Example - Projects**](https://doc.rust-lang.org/rust-by-example/std_misc.html)
- [**Rustlings - Projects**](https://github.com/rust-lang/rustlings) - Exercícios interativos
- [**Rust Reference - Projects**](https://doc.rust-lang.org/reference/items.html)

### **Comunidades e Fóruns**
- [**Reddit r/rust**](https://reddit.com/r/rust) - Comunidade ativa
- [**Rust Users Forum**](https://users.rust-lang.org/) - Fórum oficial
- [**Stack Overflow**](https://stackoverflow.com/questions/tagged/rust) - Perguntas e respostas
- [**Rust Discord**](https://discord.gg/rust-lang) - Chat em tempo real

## 📖 **Índice do Módulo**

- [**Objetivos de Aprendizagem**](#-objetivos-de-aprendizagem)
- [**Pré-requisitos**](#-pré-requisitos)
- [**Recursos de Aprendizado**](#-recursos-de-aprendizado)
- [**Projetos do Módulo**](#-projetos-do-módulo)
- [**Tutorial Prático**](#-tutorial-prático-sistema-completo-integrado)
- [**Atividades Práticas**](#-atividades-práticas)
- [**Exercícios de Fixação**](#-exercícios-de-fixação)
- [**Próximos Passos**](#-próximos-passos)
- [**Navegação**](#-navegação)

## 📚 Projetos do Módulo

### 1. Sistema de Banco de Dados Simples

```rust
// src/database.rs
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    pub id: u32,
    pub data: HashMap<String, String>,
}

pub struct Database {
    records: HashMap<u32, Record>,
    next_id: u32,
    file_path: String,
}

impl Database {
    pub fn new(file_path: &str) -> Self {
        Database {
            records: HashMap::new(),
            next_id: 1,
            file_path: file_path.to_string(),
        }
    }
    
    pub fn insert(&mut self, data: HashMap<String, String>) -> u32 {
        let id = self.next_id;
        let record = Record { id, data };
        self.records.insert(id, record);
        self.next_id += 1;
        id
    }
    
    pub fn get(&self, id: u32) -> Option<&Record> {
        self.records.get(&id)
    }
    
    pub fn update(&mut self, id: u32, data: HashMap<String, String>) -> bool {
        if let Some(record) = self.records.get_mut(&id) {
            record.data = data;
            true
        } else {
            false
        }
    }
    
    pub fn delete(&mut self, id: u32) -> bool {
        self.records.remove(&id).is_some()
    }
    
    pub fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(&self.records)?;
        fs::write(&self.file_path, json)?;
        Ok(())
    }
    
    pub fn load_from_file(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if std::path::Path::new(&self.file_path).exists() {
            let content = fs::read_to_string(&self.file_path)?;
            let records: HashMap<u32, Record> = serde_json::from_str(&content)?;
            self.records = records;
            self.next_id = self.records.keys().max().unwrap_or(&0) + 1;
        }
        Ok(())
    }
}
```

### 2. Cliente HTTP

```rust
// src/http_client.rs
use reqwest;
use serde_json::Value;
use std::collections::HashMap;

pub struct HttpClient {
    client: reqwest::Client,
    base_url: String,
}

impl HttpClient {
    pub fn new(base_url: &str) -> Self {
        HttpClient {
            client: reqwest::Client::new(),
            base_url: base_url.to_string(),
        }
    }
    
    pub async fn get(&self, endpoint: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = self.client.get(&url).send().await?;
        let json: Value = response.json().await?;
        Ok(json)
    }
    
    pub async fn post(&self, endpoint: &str, data: &Value) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = self.client.post(&url).json(data).send().await?;
        let json: Value = response.json().await?;
        Ok(json)
    }
    
    pub async fn put(&self, endpoint: &str, data: &Value) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = self.client.put(&url).json(data).send().await?;
        let json: Value = response.json().await?;
        Ok(json)
    }
    
    pub async fn delete(&self, endpoint: &str) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/{}", self.base_url, endpoint);
        self.client.delete(&url).send().await?;
        Ok(())
    }
}
```

### 3. Jogo da Vida de Conway

```rust
// src/game_of_life.rs
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
}

pub struct GameOfLife {
    cells: HashSet<Cell>,
    width: usize,
    height: usize,
}

impl GameOfLife {
    pub fn new(width: usize, height: usize) -> Self {
        GameOfLife {
            cells: HashSet::new(),
            width,
            height,
        }
    }
    
    pub fn set_cell(&mut self, x: i32, y: i32, alive: bool) {
        let cell = Cell { x, y };
        if alive {
            self.cells.insert(cell);
        } else {
            self.cells.remove(&cell);
        }
    }
    
    pub fn is_alive(&self, x: i32, y: i32) -> bool {
        self.cells.contains(&Cell { x, y })
    }
    
    pub fn count_neighbors(&self, x: i32, y: i32) -> usize {
        let mut count = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if self.is_alive(x + dx, y + dy) {
                    count += 1;
                }
            }
        }
        count
    }
    
    pub fn next_generation(&mut self) {
        let mut new_cells = HashSet::new();
        
        // Considerar todas as células vivas e suas vizinhas
        let mut cells_to_check = HashSet::new();
        for cell in &self.cells {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    cells_to_check.insert(Cell {
                        x: cell.x + dx,
                        y: cell.y + dy,
                    });
                }
            }
        }
        
        // Aplicar regras do jogo da vida
        for cell in cells_to_check {
            let neighbors = self.count_neighbors(cell.x, cell.y);
            let is_alive = self.is_alive(cell.x, cell.y);
            
            let should_live = match (is_alive, neighbors) {
                (true, 2) | (true, 3) => true,  // Sobrevive
                (false, 3) => true,             // Nasce
                _ => false,                     // Morre ou permanece morta
            };
            
            if should_live {
                new_cells.insert(cell);
            }
        }
        
        self.cells = new_cells;
    }
    
    pub fn render(&self) -> String {
        let mut output = String::new();
        
        for y in 0..self.height as i32 {
            for x in 0..self.width as i32 {
                if self.is_alive(x, y) {
                    output.push('█');
                } else {
                    output.push(' ');
                }
            }
            output.push('\n');
        }
        
        output
    }
    
    pub fn add_pattern(&mut self, pattern: &[&str], start_x: i32, start_y: i32) {
        for (y, row) in pattern.iter().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                if ch == '█' {
                    self.set_cell(start_x + x as i32, start_y + y as i32, true);
                }
            }
        }
    }
}
```

### 4. Sistema Integrado Completo

```rust
// src/main.rs
mod database;
mod http_client;
mod game_of_life;
mod server;

use std::io;
use database::Database;
use game_of_life::GameOfLife;

fn main() {
    println!("=== Sistema Integrado Rust ===");
    
    let mut db = Database::new("data.json");
    db.load_from_file().unwrap();
    
    let mut game = GameOfLife::new(50, 20);
    
    // Adicionar padrão inicial
    let glider = [
        " █ ",
        "█ █",
        " ██",
    ];
    game.add_pattern(&glider, 10, 10);
    
    loop {
        mostrar_menu();
        
        let escolha = ler_escolha();
        
        match escolha {
            1 => gerenciar_dados(&mut db),
            2 => jogar_game_of_life(&mut game),
            3 => executar_teste_performance(),
            4 => {
                db.save_to_file().unwrap();
                println!("Dados salvos. Saindo...");
                break;
            }
            _ => println!("Opção inválida!"),
        }
    }
}

fn mostrar_menu() {
    println!("\n=== MENU PRINCIPAL ===");
    println!("1. Gerenciar Dados");
    println!("2. Jogo da Vida");
    println!("3. Teste de Performance");
    println!("4. Sair");
    print!("Escolha uma opção: ");
}

fn ler_escolha() -> u32 {
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Erro ao ler entrada");
    entrada.trim().parse().unwrap_or(0)
}

fn gerenciar_dados(db: &mut Database) {
    println!("\n=== GERENCIAMENTO DE DADOS ===");
    
    loop {
        println!("1. Inserir dados");
        println!("2. Buscar dados");
        println!("3. Listar todos");
        println!("4. Voltar");
        print!("Escolha: ");
        
        let escolha = ler_escolha();
        
        match escolha {
            1 => {
                let mut data = std::collections::HashMap::new();
                println!("Digite a chave:");
                let key = ler_string();
                println!("Digite o valor:");
                let value = ler_string();
                data.insert(key, value);
                
                let id = db.insert(data);
                println!("Dados inseridos com ID: {}", id);
            }
            2 => {
                println!("Digite o ID:");
                let id = ler_numero();
                if let Some(record) = db.get(id) {
                    println!("Dados encontrados: {:?}", record);
                } else {
                    println!("Dados não encontrados!");
                }
            }
            3 => {
                println!("Todos os dados:");
                for (id, record) in &db.records {
                    println!("ID {}: {:?}", id, record);
                }
            }
            4 => break,
            _ => println!("Opção inválida!"),
        }
    }
}

fn jogar_game_of_life(game: &mut GameOfLife) {
    println!("\n=== JOGO DA VIDA ===");
    
    loop {
        println!("{}", game.render());
        println!("Pressione Enter para próxima geração (ou 'q' para sair):");
        
        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).expect("Erro ao ler entrada");
        
        if entrada.trim() == "q" {
            break;
        }
        
        game.next_generation();
    }
}

fn executar_teste_performance() {
    println!("\n=== TESTE DE PERFORMANCE ===");
    
    let start = std::time::Instant::now();
    
    // Teste de cálculo intensivo
    let mut soma = 0u64;
    for i in 1..1_000_000 {
        soma += i * i;
    }
    
    let duration = start.elapsed();
    println!("Cálculo concluído em: {:?}", duration);
    println!("Soma: {}", soma);
    
    // Teste de collections
    let start = std::time::Instant::now();
    
    let mut vec = Vec::new();
    for i in 1..100_000 {
        vec.push(i);
    }
    
    let duration = start.elapsed();
    println!("Criação de Vec com 100k elementos: {:?}", duration);
}

fn ler_string() -> String {
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Erro ao ler entrada");
    entrada.trim().to_string()
}

fn ler_numero() -> u32 {
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Erro ao ler entrada");
    entrada.trim().parse().unwrap_or(0)
}
```

## 🎯 Projetos de Portfólio

### 1. Sistema de Gerenciamento de Tarefas
- Interface web com Rocket
- Banco de dados SQLite
- Autenticação de usuários
- API REST completa

### 2. Jogo Multiplayer
- Servidor de jogo com Tokio
- Cliente web com WebSocket
- Sistema de salas
- Chat em tempo real

### 3. Compilador Simples
- Parser de expressões matemáticas
- Gerador de código
- Otimizações básicas
- Interface de linha de comando

### 4. Sistema de Monitoramento
- Coleta de métricas do sistema
- Dashboard web
- Alertas automáticos
- Histórico de dados

## 📝 Checklist Final

- [ ] Dominou ownership e borrowing
- [ ] Implementou structs e enums
- [ ] Usou collections e iterators
- [ ] Tratou erros com Result/Option
- [ ] Criou programas concorrentes
- [ ] Desenvolveu com traits e generics
- [ ] Entendeu lifetimes
- [ ] Publicou crates
- [ ] Criou projetos completos

## 🎓 Certificação

Parabéns! Você completou o curso completo de Rust. Você agora é capaz de:

- Desenvolver aplicações Rust profissionais
- Trabalhar com sistemas embarcados
- Criar bibliotecas e crates
- Contribuir para projetos open source
- Resolver problemas complexos com Rust

## 🧭 **Navegação**

### **📚 Material de Apoio**
- [**README Principal**](../../README.md) - Visão geral do curso
- [**Tutoriais Detalhados**](../../TUTORIAIS.md) - Guia completo de tutoriais
- [**Módulo 9: Cargo/Crates**](../modulo-09-cargo-crates/README.md) - Módulo anterior
- [**Módulo Embarcados**](../modulo-embarcados/README.md) - Desenvolvimento IoT

### **🔗 Links Úteis**
- [Comunidade Rust Brasil](https://github.com/rust-br)
- [Documentação Oficial](https://doc.rust-lang.org/)
- [Rust Playground](https://play.rust-lang.org/)
- [Crates.io](https://crates.io/)

### **📖 Documentação Oficial**
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)
- [Cargo Book](https://doc.rust-lang.org/cargo/)

---

**Professor:** Jackson Sá  
**ETEC Bento Quirino - Campinas/SP**
