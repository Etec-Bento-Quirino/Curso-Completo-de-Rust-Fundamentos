# Módulo 8: Lifetimes e Advanced Types

## 🎯 Objetivos de Aprendizagem

- Dominar lifetime annotations
- Usar smart pointers (Box, Rc, RefCell)
- Implementar sistemas de cache eficientes
- Otimizar performance com tipos avançados

## 📚 Conteúdo Teórico

### 8.1 Lifetimes - Gerenciamento de Referências

```mermaid
graph TD
    A[Lifetimes] --> B[Anotações]
    A --> C[Inferência]
    A --> D[Elision]
    
    B --> E['a, 'b, 'static]
    C --> F[Automatic]
    D --> G[Rules]
    
    style A fill:#e1f5fe
    style B fill:#f3e5f5
    style C fill:#fff3e0
    style D fill:#e8f5e8
```

### 8.2 Smart Pointers

```rust
// exemplos/lifetimes_advanced.rs
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

// Lifetime annotation
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Struct com lifetime
struct Excerpt<'a> {
    part: &'a str,
}

// Smart pointers
struct Node<T> {
    value: T,
    children: Vec<Rc<RefCell<Node<T>>>>,
    parent: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            value,
            children: Vec::new(),
            parent: None,
        }))
    }
    
    fn add_child(&mut self, child: Rc<RefCell<Node<T>>>) {
        child.borrow_mut().parent = Some(Rc::new(RefCell::new(self)));
        self.children.push(child);
    }
}

fn main() {
    // Lifetime example
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("A string mais longa é {}", result);
    
    // Excerpt example
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let excerpt = Excerpt { part: first_sentence };
    println!("Excerpt: {}", excerpt.part);
    
    // Smart pointers example
    let root = Node::new(1);
    let child1 = Node::new(2);
    let child2 = Node::new(3);
    
    {
        let mut root_borrow = root.borrow_mut();
        root_borrow.add_child(child1.clone());
        root_borrow.add_child(child2.clone());
    }
    
    println!("Root value: {}", root.borrow().value);
    println!("Children count: {}", root.borrow().children.len());
}
```

## 🎯 Tutorial Prático: Sistema de Cache

### Implementação Completa

```rust
// src/main.rs
mod cache;
mod lru_cache;

use cache::Cache;
use lru_cache::LRUCache;

fn main() {
    println!("=== Sistema de Cache ===");
    
    let mut cache = LRUCache::new(3);
    
    cache.put("key1", "value1");
    cache.put("key2", "value2");
    cache.put("key3", "value3");
    
    println!("Cache após inserções: {:?}", cache);
    
    if let Some(value) = cache.get("key1") {
        println!("Valor encontrado: {}", value);
    }
    
    cache.put("key4", "value4"); // Remove key1 (LRU)
    
    println!("Cache após inserir key4: {:?}", cache);
}
```

```rust
// src/cache.rs
use std::collections::HashMap;
use std::rc::Rc;

pub trait Cache<K, V> {
    fn get(&self, key: &K) -> Option<&V>;
    fn put(&mut self, key: K, value: V);
    fn remove(&mut self, key: &K) -> Option<V>;
    fn clear(&mut self);
    fn size(&self) -> usize;
}

pub struct SimpleCache<K, V> {
    data: HashMap<K, V>,
    max_size: usize,
}

impl<K, V> SimpleCache<K, V>
where
    K: std::hash::Hash + Eq + Clone,
{
    pub fn new(max_size: usize) -> Self {
        SimpleCache {
            data: HashMap::new(),
            max_size,
        }
    }
}

impl<K, V> Cache<K, V> for SimpleCache<K, V>
where
    K: std::hash::Hash + Eq + Clone,
{
    fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }
    
    fn put(&mut self, key: K, value: V) {
        if self.data.len() >= self.max_size {
            // Remove o primeiro elemento (FIFO)
            if let Some(first_key) = self.data.keys().next().cloned() {
                self.data.remove(&first_key);
            }
        }
        self.data.insert(key, value);
    }
    
    fn remove(&mut self, key: &K) -> Option<V> {
        self.data.remove(key)
    }
    
    fn clear(&mut self) {
        self.data.clear();
    }
    
    fn size(&self) -> usize {
        self.data.len()
    }
}
```

```rust
// src/lru_cache.rs
use std::collections::HashMap;
use std::rc::Rc;

use crate::cache::Cache;

struct LRUNode<K, V> {
    key: K,
    value: V,
    prev: Option<Rc<RefCell<LRUNode<K, V>>>>,
    next: Option<Rc<RefCell<LRUNode<K, V>>>>,
}

pub struct LRUCache<K, V> {
    capacity: usize,
    cache: HashMap<K, Rc<RefCell<LRUNode<K, V>>>>,
    head: Option<Rc<RefCell<LRUNode<K, V>>>>,
    tail: Option<Rc<RefCell<LRUNode<K, V>>>>,
}

impl<K, V> LRUCache<K, V>
where
    K: std::hash::Hash + Eq + Clone,
{
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            cache: HashMap::new(),
            head: None,
            tail: None,
        }
    }
    
    fn add_to_head(&mut self, node: Rc<RefCell<LRUNode<K, V>>>) {
        if let Some(head) = &self.head {
            head.borrow_mut().prev = Some(node.clone());
        }
        
        node.borrow_mut().prev = None;
        node.borrow_mut().next = self.head.clone();
        self.head = Some(node);
        
        if self.tail.is_none() {
            self.tail = self.head.clone();
        }
    }
    
    fn remove_node(&mut self, node: Rc<RefCell<LRUNode<K, V>>>) {
        let prev = node.borrow().prev.clone();
        let next = node.borrow().next.clone();
        
        if let Some(prev) = prev {
            prev.borrow_mut().next = next.clone();
        } else {
            self.head = next.clone();
        }
        
        if let Some(next) = next {
            next.borrow_mut().prev = prev;
        } else {
            self.tail = prev;
        }
    }
}

impl<K, V> Cache<K, V> for LRUCache<K, V>
where
    K: std::hash::Hash + Eq + Clone,
{
    fn get(&self, key: &K) -> Option<&V> {
        if let Some(node) = self.cache.get(key) {
            // Move to head (would need &mut self)
            Some(&node.borrow().value)
        } else {
            None
        }
    }
    
    fn put(&mut self, key: K, value: V) {
        if let Some(node) = self.cache.get(&key) {
            // Update existing node
            node.borrow_mut().value = value;
            // Move to head (simplified)
            return;
        }
        
        if self.cache.len() >= self.capacity {
            // Remove tail
            if let Some(tail) = &self.tail {
                let tail_key = tail.borrow().key.clone();
                self.cache.remove(&tail_key);
                self.remove_node(tail.clone());
            }
        }
        
        // Add new node
        let new_node = Rc::new(RefCell::new(LRUNode {
            key: key.clone(),
            value,
            prev: None,
            next: None,
        }));
        
        self.add_to_head(new_node.clone());
        self.cache.insert(key, new_node);
    }
    
    fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(node) = self.cache.remove(key) {
            self.remove_node(node.clone());
            Some(node.borrow().value)
        } else {
            None
        }
    }
    
    fn clear(&mut self) {
        self.cache.clear();
        self.head = None;
        self.tail = None;
    }
    
    fn size(&self) -> usize {
        self.cache.len()
    }
}
```

## 🎯 Atividades Práticas

### Atividade 1: Gerenciador de Memória
Implemente um gerenciador de memória com smart pointers.

### Atividade 2: Sistema de Referências
Crie um sistema que gerencie referências circulares.

### Atividade 3: Cache Distribuído
Desenvolva um cache que funcione entre múltiplas threads.

---

**Professor:** Jackson Sá  
**ETEC Bento Quirino - Campinas/SP**
