fn main() {
    println!("=== Exemplo: Variáveis e Mutabilidade ===");
    
    // Variável imutável (padrão em Rust)
    let x = 5;
    println!("O valor de x é: {}", x);
    
    // Tentativa de modificar variável imutável (causaria erro)
    // x = 6; // ❌ Erro de compilação!
    
    // Variável mutável
    let mut y = 10;
    println!("O valor inicial de y é: {}", y);
    
    y = 15;
    println!("O valor final de y é: {}", y);
    
    // Shadowing (sombreamento) - redefinir variável com mesmo nome
    let z = 5;
    println!("Primeiro valor de z: {}", z);
    
    let z = z + 1; // z agora é 6
    println!("Segundo valor de z: {}", z);
    
    let z = z * 2; // z agora é 12
    println!("Terceiro valor de z: {}", z);
    
    // Shadowing com tipo diferente
    let spaces = "   "; // string
    let spaces = spaces.len(); // número
    println!("Número de espaços: {}", spaces);
    
    // Constantes (sempre imutáveis, avaliadas em tempo de compilação)
    const MAX_POINTS: u32 = 100_000;
    println!("Pontuação máxima: {}", MAX_POINTS);
    
    // Demonstração de escopo
    let x = 10;
    println!("x no escopo principal: {}", x);
    
    {
        let x = 20; // x sombreia a variável do escopo externo
        println!("x no escopo interno: {}", x);
    }
    
    println!("x após o escopo interno: {}", x); // volta ao valor original
}
