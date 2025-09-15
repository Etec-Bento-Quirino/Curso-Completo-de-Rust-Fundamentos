fn main() {
    println!("=== Exemplo: Funções ===");
    
    // Chamando funções
    let resultado = somar(5, 3);
    println!("5 + 3 = {}", resultado);
    
    let quadrado = elevar_ao_quadrado(4);
    println!("4² = {}", quadrado);
    
    imprimir_saudacao("Rust");
    
    let (soma, produto) = operacoes(6, 7);
    println!("Soma: {}, Produto: {}", soma, produto);
    
    let fatorial_5 = fatorial(5);
    println!("5! = {}", fatorial_5);
    
    let fibonacci_10 = fibonacci(10);
    println!("Fibonacci(10) = {}", fibonacci_10);
    
    // Função com múltiplos parâmetros
    let media = calcular_media(10, 20, 30);
    println!("Média de 10, 20, 30: {}", media);
    
    // Função que retorna boolean
    let eh_par = verificar_par(8);
    println!("8 é par? {}", eh_par);
    
    // Função com parâmetros opcionais (usando Option)
    let resultado_opcional = dividir_seguro(10, 2);
    match resultado_opcional {
        Some(valor) => println!("10 / 2 = {}", valor),
        None => println!("Divisão por zero!"),
    }
    
    let resultado_invalido = dividir_seguro(10, 0);
    match resultado_invalido {
        Some(valor) => println!("Resultado: {}", valor),
        None => println!("Divisão por zero!"),
    }
}

// Função com parâmetros e retorno explícito
fn somar(a: i32, b: i32) -> i32 {
    a + b
}

// Função com retorno implícito (última expressão sem ponto e vírgula)
fn elevar_ao_quadrado(x: i32) -> i32 {
    x * x
}

// Função sem retorno (unit type - ())
fn imprimir_saudacao(nome: &str) {
    println!("Olá, {}!", nome);
}

// Função que retorna múltiplos valores (tupla)
fn operacoes(a: i32, b: i32) -> (i32, i32) {
    (a + b, a * b)
}

// Função recursiva - Fatorial
fn fatorial(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        n * fatorial(n - 1)
    }
}

// Função recursiva - Fibonacci
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// Função com múltiplos parâmetros
fn calcular_media(a: f64, b: f64, c: f64) -> f64 {
    (a + b + c) / 3.0
}

// Função que retorna boolean
fn verificar_par(numero: i32) -> bool {
    numero % 2 == 0
}

// Função que retorna Option para lidar com casos de erro
fn dividir_seguro(a: f64, b: f64) -> Option<f64> {
    if b != 0.0 {
        Some(a / b)
    } else {
        None
    }
}

// Função com parâmetros de referência
fn dobrar_valor(valor: &mut i32) {
    *valor *= 2;
}

// Função que aceita slice
fn imprimir_array(numeros: &[i32]) {
    println!("Array: {:?}", numeros);
    println!("Primeiro elemento: {}", numeros[0]);
    println!("Tamanho: {}", numeros.len());
}

// Função genérica (será explicada em módulos posteriores)
fn _primeiro_elemento<T>(lista: &[T]) -> Option<&T> {
    lista.first()
}

// Função com closure como parâmetro
fn aplicar_operacao<F>(a: i32, b: i32, operacao: F) -> i32 
where 
    F: Fn(i32, i32) -> i32 
{
    operacao(a, b)
}

// Exemplo de uso de closures
fn _exemplo_closures() {
    let soma = |a, b| a + b;
    let multiplicacao = |a, b| a * b;
    
    let resultado1 = aplicar_operacao(5, 3, soma);
    let resultado2 = aplicar_operacao(5, 3, multiplicacao);
    
    println!("Soma: {}, Multiplicação: {}", resultado1, resultado2);
}
