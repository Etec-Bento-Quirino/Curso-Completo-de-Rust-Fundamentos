fn main() {
    println!("=== Exemplo: Ownership Básico ===");
    
    // Ownership com String (heap)
    let s1 = String::from("hello");
    println!("s1: {}", s1);
    
    // Movimento de ownership
    let s2 = s1; // s1 é movido para s2
    // println!("s1: {}", s1); // ❌ ERRO! s1 não é mais válido
    println!("s2: {}", s2);
    
    // Clone (cópia profunda)
    let s3 = s2.clone();
    println!("s2: {}", s2); // s2 ainda é válido
    println!("s3: {}", s3);
    
    // Ownership com tipos primitivos (stack)
    let x = 5;
    let y = x; // Cópia automática (Copy trait)
    println!("x: {}, y: {}", x, y); // Ambos válidos
    
    // Demonstração de escopo
    let s4 = String::from("escopo");
    {
        let s5 = String::from("interno");
        println!("s5: {}", s5);
        // s5 sai de escopo aqui e é liberado
    }
    println!("s4: {}", s4);
    // s4 sai de escopo aqui e é liberado
    
    // Demonstração com múltiplas Strings
    println!("\n--- Múltiplas Strings ---");
    let mut strings = Vec::new();
    
    for i in 0..3 {
        let s = String::from(format!("string {}", i));
        strings.push(s);
    }
    
    println!("Strings criadas:");
    for (i, s) in strings.iter().enumerate() {
        println!("  {}. {}", i, s);
    }
    
    // Demonstração de ownership em funções
    println!("\n--- Ownership em Funções ---");
    let s_original = String::from("original");
    println!("Antes da função: {}", s_original);
    
    let s_retornada = move_string(s_original);
    println!("Depois da função: {}", s_retornada);
    
    // Demonstração com Copy types
    println!("\n--- Copy Types ---");
    let numero = 42;
    println!("Antes: {}", numero);
    
    copy_number(numero);
    println!("Depois: {}", numero); // Ainda válido!
}

fn move_string(s: String) -> String {
    println!("Dentro da função: {}", s);
    s // Retorna ownership
}

fn copy_number(n: i32) {
    println!("Copiando número: {}", n);
    // n é copiado, não movido
}
