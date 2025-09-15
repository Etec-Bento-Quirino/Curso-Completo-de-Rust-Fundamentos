#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
    NewYork,
    Texas,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    println!("=== Exemplo: Pattern Matching ===");
    
    // Match com enums
    let coin = Coin::Quarter(UsState::California);
    println!("Valor da moeda: {}", value_in_cents(coin));
    
    // Match com Option
    let some_number = Some(5);
    let absent_number: Option<i32> = None;
    
    println!("Some number: {}", plus_one(some_number));
    println!("Absent number: {}", plus_one(absent_number));
    
    // Match com ranges
    let number = 5;
    match number {
        1..=5 => println!("Número entre 1 e 5"),
        6..=10 => println!("Número entre 6 e 10"),
        _ => println!("Outro número"),
    }
    
    // Match com structs
    let point = Point { x: 0, y: 7 };
    match point {
        Point { x, y: 0 } => println!("No eixo X em {}", x),
        Point { x: 0, y } => println!("No eixo Y em {}", y),
        Point { x, y } => println!("Em ({}, {})", x, y),
    }
    
    // Match com guards
    let pair = (4, 5);
    match pair {
        (x, y) if x == y => println!("Iguais"),
        (x, y) if x + y == 10 => println!("Soma é 10"),
        (x, y) => println!("Outros valores: ({}, {})", x, y),
    }
    
    // If let
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("Máximo configurado: {}", max);
    }
    
    // If let com else
    let mut count = 0;
    let coin = Coin::Quarter(UsState::California);
    if let Coin::Quarter(state) = coin {
        println!("Moeda do estado {:?}!", state);
    } else {
        count += 1;
    }
    
    // While let
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    println!("Desempilhando:");
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
    
    // For loops (também usam pattern matching)
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} está no índice {}", value, index);
    }
    
    // Match com múltiplos padrões
    let number = 42;
    match number {
        1 | 3 | 5 | 7 | 9 => println!("Ímpar"),
        2 | 4 | 6 | 8 | 10 => println!("Par"),
        _ => println!("Outro número"),
    }
    
    // Match com destructuring de tuplas
    let tuple = (1, 2, 3);
    match tuple {
        (1, 2, 3) => println!("Tupla exata"),
        (x, 2, 3) => println!("Primeiro elemento: {}", x),
        (1, y, 3) => println!("Segundo elemento: {}", y),
        (1, 2, z) => println!("Terceiro elemento: {}", z),
        (x, y, z) => println!("Elementos: {}, {}, {}", x, y, z),
    }
    
    // Match com enums complexos
    let message = Message::Move { x: 10, y: 20 };
    process_message(message);
    
    let message2 = Message::Write(String::from("Hello World"));
    process_message(message2);
    
    // Match com Option e Result
    let resultado = dividir(10.0, 2.0);
    match resultado {
        Some(valor) => println!("Resultado da divisão: {}", valor),
        None => println!("Divisão por zero!"),
    }
    
    let resultado_erro = dividir(10.0, 0.0);
    match resultado_erro {
        Some(valor) => println!("Resultado: {}", valor),
        None => println!("Erro na divisão!"),
    }
    
    // Demonstração de match exaustivo
    let opcao = Some(42);
    match opcao {
        Some(valor) => println!("Valor: {}", valor),
        None => println!("Nenhum valor"),
        // Não precisamos de _ porque cobrimos todos os casos
    }
    
    // Match com referências
    let valor = 42;
    match &valor {
        &42 => println!("É 42!"),
        &x => println!("É {}", x),
    }
    
    // Match com slices
    let array = [1, 2, 3, 4, 5];
    match array {
        [1, 2, 3, ..] => println!("Começa com 1, 2, 3"),
        [.., 4, 5] => println!("Termina com 4, 5"),
        [x, y, z, ..] => println!("Primeiros três: {}, {}, {}", x, y, z),
        _ => println!("Outro padrão"),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Moeda da sorte!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Moeda do estado {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> i32 {
    match x {
        None => 0,
        Some(i) => i + 1,
    }
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Encerrando aplicação..."),
        Message::Move { x, y } => println!("Movendo para ({}, {})", x, y),
        Message::Write(text) => println!("Escrevendo: '{}'", text),
        Message::ChangeColor(r, g, b) => {
            println!("Mudando cor para RGB({}, {}, {})", r, g, b);
            if r > 200 && g < 100 && b < 100 {
                println!("Cor vermelha detectada!");
            }
        }
    }
}

fn dividir(a: f64, b: f64) -> Option<f64> {
    if b != 0.0 {
        Some(a / b)
    } else {
        None
    }
}

// Função que demonstra diferentes tipos de pattern matching
fn demonstrar_patterns() {
    println!("\n--- Demonstração de Patterns ---");
    
    // Pattern com guard
    let numero = 42;
    match numero {
        x if x < 10 => println!("Número pequeno: {}", x),
        x if x < 100 => println!("Número médio: {}", x),
        x => println!("Número grande: {}", x),
    }
    
    // Pattern com binding
    let ponto = Point { x: 10, y: 20 };
    match ponto {
        Point { x: 0, y } => println!("No eixo Y: {}", y),
        Point { x, y: 0 } => println!("No eixo X: {}", x),
        Point { x, y } => println!("Ponto: ({}, {})", x, y),
    }
    
    // Pattern com @ binding
    let valor = 42;
    match valor {
        x @ 1..=50 => println!("Valor entre 1 e 50: {}", x),
        x @ 51..=100 => println!("Valor entre 51 e 100: {}", x),
        x => println!("Valor fora do range: {}", x),
    }
}
