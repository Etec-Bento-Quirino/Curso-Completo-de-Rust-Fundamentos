mod calculadora;
mod operacoes;

use std::io;

fn main() {
    println!("=== Calculadora Rust ===");
    println!("Bem-vindo à calculadora desenvolvida em Rust!");
    
    loop {
        mostrar_menu();
        
        let escolha = ler_escolha();
        
        match escolha {
            1 => calculadora::executar_operacao("soma", operacoes::somar),
            2 => calculadora::executar_operacao("subtração", operacoes::subtrair),
            3 => calculadora::executar_operacao("multiplicação", operacoes::multiplicar),
            4 => calculadora::executar_operacao("divisão", operacoes::dividir),
            5 => calculadora::executar_operacao("potenciação", operacoes::potencia),
            6 => calculadora::executar_operacao("raiz quadrada", operacoes::raiz_quadrada),
            7 => {
                println!("Obrigado por usar a calculadora!");
                break;
            }
            _ => {
                println!("Opção inválida! Tente novamente.");
                continue;
            }
        }
        
        println!(); // Linha em branco para separar operações
    }
}

fn mostrar_menu() {
    println!("\nEscolha uma operação:");
    println!("1. Soma");
    println!("2. Subtração");
    println!("3. Multiplicação");
    println!("4. Divisão");
    println!("5. Potenciação");
    println!("6. Raiz Quadrada");
    println!("7. Sair");
    print!("Digite sua escolha (1-7): ");
}

fn ler_escolha() -> u32 {
    let mut entrada = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .expect("Falha ao ler entrada");
    
    entrada.trim().parse().unwrap_or(0)
}
