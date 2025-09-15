fn main() {
    println!("=== Exemplo: Controle de Fluxo ===");
    
    // IF/ELSE
    exemplo_if_else();
    
    // LOOP
    exemplo_loop();
    
    // WHILE
    exemplo_while();
    
    // FOR
    exemplo_for();
    
    // MATCH (pattern matching)
    exemplo_match();
}

fn exemplo_if_else() {
    println!("\n--- IF/ELSE ---");
    
    let numero = 7;
    
    if numero < 5 {
        println!("Condição era verdadeira");
    } else if numero < 10 {
        println!("Número está entre 5 e 9");
    } else {
        println!("Condição era falsa");
    }
    
    // If como expressão
    let condicao = true;
    let numero = if condicao { 5 } else { 6 };
    println!("O valor do número é: {}", numero);
    
    // If com múltiplas condições
    let idade = 20;
    let tem_carteira = true;
    
    if idade >= 18 && tem_carteira {
        println!("Pode dirigir!");
    } else if idade >= 18 {
        println!("Precisa de carteira de motorista");
    } else {
        println!("Muito jovem para dirigir");
    }
    
    // If com operadores lógicos
    let temperatura = 25;
    let umidade = 60;
    
    if temperatura > 30 || umidade > 80 {
        println!("Clima quente e úmido!");
    } else if temperatura > 25 && umidade < 50 {
        println!("Clima agradável");
    } else {
        println!("Clima normal");
    }
}

fn exemplo_loop() {
    println!("\n--- LOOP ---");
    
    // Loop infinito com break
    let mut contador = 0;
    loop {
        contador += 1;
        println!("Contador: {}", contador);
        
        if contador == 3 {
            break;
        }
    }
    println!("Loop terminou. Contador final: {}", contador);
    
    // Loop com valor de retorno
    let mut contador = 0;
    let resultado = loop {
        contador += 1;
        
        if contador == 10 {
            break contador * 2; // Retorna valor
        }
    };
    println!("Resultado do loop: {}", resultado);
    
    // Loop com labels (rótulos) para loops aninhados
    let mut contador_externo = 0;
    'externo: loop {
        println!("Contador externo: {}", contador_externo);
        let mut contador_interno = 0;
        
        loop {
            contador_interno += 1;
            println!("  Contador interno: {}", contador_interno);
            
            if contador_interno == 3 {
                break; // Sai apenas do loop interno
            }
            
            if contador_externo == 2 && contador_interno == 2 {
                break 'externo; // Sai do loop externo
            }
        }
        
        contador_externo += 1;
        
        if contador_externo == 3 {
            break;
        }
    }
}

fn exemplo_while() {
    println!("\n--- WHILE ---");
    
    let mut numero = 3;
    while numero != 0 {
        println!("{}!", numero);
        numero -= 1;
    }
    println!("LIFTOFF!!!");
    
    // While com condição complexa
    let mut x = 0;
    let mut y = 10;
    
    while x < 5 && y > 0 {
        println!("x: {}, y: {}", x, y);
        x += 1;
        y -= 2;
    }
    
    // While com array
    let array = [1, 2, 3, 4, 5];
    let mut indice = 0;
    
    while indice < array.len() {
        println!("array[{}] = {}", indice, array[indice]);
        indice += 1;
    }
}

fn exemplo_for() {
    println!("\n--- FOR ---");
    
    // For com array
    let array = [10, 20, 30, 40, 50];
    println!("Iterando sobre array:");
    for elemento in array.iter() {
        println!("O valor é: {}", elemento);
    }
    
    // For com range
    println!("\nContagem regressiva:");
    for numero in (1..4).rev() {
        println!("{}!", numero);
    }
    println!("LIFTOFF!!!");
    
    // For com range inclusivo
    println!("\nContagem de 1 a 5:");
    for numero in 1..=5 {
        println!("Número: {}", numero);
    }
    
    // For com enumerate (índice e valor)
    println!("\nArray com índices:");
    for (indice, valor) in array.iter().enumerate() {
        println!("Índice {}: {}", indice, valor);
    }
    
    // For com múltiplos ranges
    println!("\nTabuada do 2:");
    for i in 1..=10 {
        println!("2 x {} = {}", i, 2 * i);
    }
    
    // For com step
    println!("\nNúmeros pares de 0 a 10:");
    for numero in (0..=10).step_by(2) {
        println!("Número par: {}", numero);
    }
}

fn exemplo_match() {
    println!("\n--- MATCH (Pattern Matching) ---");
    
    let numero = 3;
    
    match numero {
        1 => println!("Um"),
        2 => println!("Dois"),
        3 => println!("Três"),
        4 => println!("Quatro"),
        5 => println!("Cinco"),
        _ => println!("Outro número"),
    }
    
    // Match com ranges
    let idade = 25;
    let categoria = match idade {
        0..=12 => "Criança",
        13..=19 => "Adolescente",
        20..=59 => "Adulto",
        60..=120 => "Idoso",
        _ => "Idade inválida",
    };
    println!("Idade {}: {}", idade, categoria);
    
    // Match com múltiplos padrões
    let numero = 4;
    match numero {
        1 | 3 | 5 | 7 | 9 => println!("Ímpar"),
        2 | 4 | 6 | 8 | 10 => println!("Par"),
        _ => println!("Fora do range"),
    }
    
    // Match com tuplas
    let coordenada = (0, 5);
    match coordenada {
        (0, 0) => println!("Origem"),
        (0, y) => println!("No eixo Y, y = {}", y),
        (x, 0) => println!("No eixo X, x = {}", x),
        (x, y) => println!("Em ({}, {})", x, y),
    }
    
    // Match com Option
    let opcional = Some(42);
    match opcional {
        Some(valor) => println!("Valor encontrado: {}", valor),
        None => println!("Nenhum valor"),
    }
    
    // Match com if (guards)
    let numero = 6;
    match numero {
        x if x < 5 => println!("Menor que 5: {}", x),
        x if x < 10 => println!("Entre 5 e 9: {}", x),
        x => println!("Maior ou igual a 10: {}", x),
    }
    
    // Match com enums (será explicado em módulos posteriores)
    enum Status {
        Sucesso,
        Erro(String),
        Carregando,
    }
    
    let status = Status::Erro(String::from("Arquivo não encontrado"));
    
    match status {
        Status::Sucesso => println!("Operação bem-sucedida"),
        Status::Erro(mensagem) => println!("Erro: {}", mensagem),
        Status::Carregando => println!("Carregando..."),
    }
}
