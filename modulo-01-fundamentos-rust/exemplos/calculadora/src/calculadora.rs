use std::io;

pub fn ler_numero() -> f64 {
    loop {
        let mut entrada = String::new();
        io::stdin()
            .read_line(&mut entrada)
            .expect("Falha ao ler entrada");
        
        match entrada.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor, digite um número válido:");
                continue;
            }
        }
    }
}

pub fn executar_operacao(nome_operacao: &str, operacao: fn(f64, f64) -> f64) {
    println!("\n=== {} ===", nome_operacao.to_uppercase());
    
    println!("Digite o primeiro número:");
    let a = ler_numero();
    
    println!("Digite o segundo número:");
    let b = ler_numero();
    
    let resultado = operacao(a, b);
    println!("Resultado: {} {} {} = {:.2}", a, obter_simbolo_operacao(nome_operacao), b, resultado);
}

pub fn executar_operacao_unaria(nome_operacao: &str, operacao: fn(f64) -> f64) {
    println!("\n=== {} ===", nome_operacao.to_uppercase());
    
    println!("Digite o número:");
    let numero = ler_numero();
    
    let resultado = operacao(numero);
    println!("√{} = {:.2}", numero, resultado);
}

fn obter_simbolo_operacao(operacao: &str) -> &str {
    match operacao {
        "soma" => "+",
        "subtração" => "-",
        "multiplicação" => "×",
        "divisão" => "÷",
        "potenciação" => "^",
        _ => "?",
    }
}
