fn main() {
    println!("=== Exemplo: Referências e Borrowing ===");
    
    let s1 = String::from("hello");
    
    // Empréstimo (borrowing) com referência imutável
    let tamanho = calcula_tamanho(&s1);
    println!("O tamanho de '{}' é {}.", s1, tamanho); // s1 ainda é válido
    
    // Referência mutável
    let mut s2 = String::from("hello");
    modifica_string(&mut s2);
    println!("s2 modificado: {}", s2);
    
    // Múltiplas referências imutáveis
    let s3 = String::from("hello world");
    let r1 = &s3;
    let r2 = &s3;
    println!("r1: {}, r2: {}", r1, r2);
    
    // Referência mutável (apenas uma por vez)
    let mut s4 = String::from("hello");
    let r3 = &mut s4;
    // let r4 = &mut s4; // ❌ ERRO! Não pode ter duas referências mutáveis
    println!("r3: {}", r3);
    
    // Referência mutável após uso
    let mut s5 = String::from("hello");
    let r5 = &s5; // Referência imutável
    let r6 = &s5; // Outra referência imutável
    println!("r5: {}, r6: {}", r5, r6);
    
    let r7 = &mut s5; // Referência mutável após uso das imutáveis
    println!("r7: {}", r7);
    
    // Demonstração de escopo de referências
    println!("\n--- Escopo de Referências ---");
    let mut s6 = String::from("escopo");
    {
        let r8 = &s6;
        println!("Referência imutável: {}", r8);
        // r8 sai de escopo aqui
    }
    
    let r9 = &mut s6;
    println!("Referência mutável: {}", r9);
    
    // Demonstração com arrays
    println!("\n--- Borrowing com Arrays ---");
    let mut array = [1, 2, 3, 4, 5];
    
    // Referência imutável para o array
    let slice_imutavel = &array[1..4];
    println!("Slice imutável: {:?}", slice_imutavel);
    
    // Referência mutável para o array
    let slice_mutavel = &mut array[2..5];
    for item in slice_mutavel.iter_mut() {
        *item *= 2;
    }
    println!("Array modificado: {:?}", array);
    
    // Demonstração de borrowing em funções
    println!("\n--- Borrowing em Funções ---");
    let s_func = String::from("função");
    processar_string(&s_func);
    println!("String original ainda válida: {}", s_func);
    
    let mut s_func_mut = String::from("mutável");
    processar_string_mut(&mut s_func_mut);
    println!("String modificada: {}", s_func_mut);
    
    // Demonstração de lifetime (conceito básico)
    println!("\n--- Lifetime Básico ---");
    let s_lifetime = String::from("lifetime");
    let resultado = retornar_referencia(&s_lifetime);
    println!("Resultado: {}", resultado);
}

fn calcula_tamanho(s: &String) -> usize {
    s.len()
    // s sai de escopo aqui, mas não libera a memória
    // porque é apenas uma referência
}

fn modifica_string(s: &mut String) {
    s.push_str(", world!");
}

fn processar_string(s: &str) {
    println!("Processando: {}", s);
    // Apenas lê a string, não a modifica
}

fn processar_string_mut(s: &mut String) {
    s.push_str(" modificada");
    println!("Processando e modificando: {}", s);
}

fn retornar_referencia(s: &str) -> &str {
    // Retorna uma referência que tem o mesmo lifetime que s
    &s[0..3]
}
