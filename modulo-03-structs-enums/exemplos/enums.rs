#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
    NewYork,
    Texas,
}

impl Message {
    fn call(&self) {
        println!("Chamando mensagem: {:?}", self);
    }
    
    fn process(&self) {
        match self {
            Message::Quit => println!("Encerrando aplicação..."),
            Message::Move { x, y } => println!("Movendo para ({}, {})", x, y),
            Message::Write(text) => println!("Escrevendo: {}", text),
            Message::ChangeColor(r, g, b) => println!("Mudando cor para RGB({}, {}, {})", r, g, b),
        }
    }
}

fn main() {
    println!("=== Exemplo: Enums ===");
    
    // Usando enums simples
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    println!("Home IP: {:?}", home);
    println!("Loopback IP: {:?}", loopback);
    
    // Usando enums com dados
    let quit = Message::Quit;
    let move_msg = Message::Move { x: 10, y: 20 };
    let write_msg = Message::Write(String::from("Hello"));
    let color_msg = Message::ChangeColor(255, 0, 0);
    
    quit.call();
    move_msg.call();
    write_msg.call();
    color_msg.call();
    
    println!("\n--- Processando Mensagens ---");
    quit.process();
    move_msg.process();
    write_msg.process();
    color_msg.process();
    
    // Trabalhando com Option
    let some_number = Some(5);
    let some_string = Some("hello");
    let absent_number: Option<i32> = None;
    
    println!("\n--- Option Examples ---");
    println!("Some number: {:?}", some_number);
    println!("Some string: {:?}", some_string);
    println!("Absent number: {:?}", absent_number);
    
    // Usando Option com match
    match some_number {
        Some(value) => println!("Valor encontrado: {}", value),
        None => println!("Nenhum valor"),
    }
    
    // Usando Result
    let success: Result<i32, String> = Ok(42);
    let error: Result<i32, String> = Err(String::from("Algo deu errado"));
    
    println!("\n--- Result Examples ---");
    println!("Success: {:?}", success);
    println!("Error: {:?}", error);
    
    // Processando Result
    match success {
        Ok(value) => println!("Sucesso: {}", value),
        Err(e) => println!("Erro: {}", e),
    }
    
    match error {
        Ok(value) => println!("Sucesso: {}", value),
        Err(e) => println!("Erro: {}", e),
    }
    
    // Função que retorna enum
    let coin = Coin::Quarter;
    println!("\n--- Coin Examples ---");
    println!("Valor da moeda: {}", value_in_cents(coin));
    
    // Demonstração com diferentes moedas
    let coins = vec![Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter];
    for coin in coins {
        println!("Moeda: {:?}, Valor: {} centavos", coin, value_in_cents(coin));
    }
    
    // Demonstração de enum com dados associados
    println!("\n--- Enum com Dados Associados ---");
    let quarter_ca = CoinWithState::Quarter(UsState::California);
    let quarter_ny = CoinWithState::Quarter(UsState::NewYork);
    
    println!("Moeda da Califórnia: {}", value_in_cents_with_state(quarter_ca));
    println!("Moeda de Nova York: {}", value_in_cents_with_state(quarter_ny));
    
    // Demonstração de enum como estado
    println!("\n--- Enum como Estado ---");
    let mut game_state = GameState::Menu;
    println!("Estado inicial: {:?}", game_state);
    
    game_state = GameState::Playing { score: 100, level: 1 };
    println!("Estado de jogo: {:?}", game_state);
    
    game_state = GameState::Paused;
    println!("Estado pausado: {:?}", game_state);
    
    game_state = GameState::GameOver { final_score: 1500 };
    println!("Estado final: {:?}", game_state);
}

#[derive(Debug)]
enum CoinWithState {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum GameState {
    Menu,
    Playing { score: u32, level: u32 },
    Paused,
    GameOver { final_score: u32 },
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Moeda da sorte!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn value_in_cents_with_state(coin: CoinWithState) -> u8 {
    match coin {
        CoinWithState::Penny => 1,
        CoinWithState::Nickel => 5,
        CoinWithState::Dime => 10,
        CoinWithState::Quarter(state) => {
            println!("Moeda do estado {:?}!", state);
            25
        }
    }
}

// Função que retorna Option
fn dividir(a: f64, b: f64) -> Option<f64> {
    if b != 0.0 {
        Some(a / b)
    } else {
        None
    }
}

// Função que retorna Result
fn abrir_arquivo(nome: &str) -> Result<String, String> {
    if nome.is_empty() {
        Err(String::from("Nome do arquivo não pode ser vazio"))
    } else {
        Ok(format!("Conteúdo do arquivo {}", nome))
    }
}

// Função que processa diferentes tipos de mensagem
fn processar_mensagem(msg: Message) {
    match msg {
        Message::Quit => println!("Encerrando..."),
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
