fn main() {
    println!("=== Exemplo: Slices ===");
    
    // String slice
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    
    println!("String completa: {}", s);
    println!("Slice 1: {}", hello);
    println!("Slice 2: {}", world);
    
    // Slice com range até o final
    let slice_fim = &s[6..];
    println!("Slice até o fim: {}", slice_fim);
    
    // Slice do início
    let slice_inicio = &s[..5];
    println!("Slice do início: {}", slice_inicio);
    
    // Slice completo
    let slice_completo = &s[..];
    println!("Slice completo: {}", slice_completo);
    
    // Array slices
    let array = [1, 2, 3, 4, 5];
    let slice_array = &array[1..4];
    println!("Array: {:?}", array);
    println!("Slice do array: {:?}", slice_array);
    
    // Função que recebe slice
    let minha_string = String::from("hello world");
    let palavra = primeira_palavra(&minha_string);
    println!("Primeira palavra: {}", palavra);
    
    // Slice como parâmetro
    let meu_array = [1, 2, 3, 4, 5];
    let soma = somar_slice(&meu_array);
    println!("Soma do array: {}", soma);
    
    // Demonstração com strings literais
    println!("\n--- String Literals ---");
    let s_literal = "hello world";
    let slice_literal = &s_literal[0..5];
    println!("String literal: {}", s_literal);
    println!("Slice da literal: {}", slice_literal);
    
    // Demonstração com Vec
    println!("\n--- Vec Slices ---");
    let mut vec = vec![1, 2, 3, 4, 5];
    let slice_vec = &vec[1..4];
    println!("Vec: {:?}", vec);
    println!("Slice do Vec: {:?}", slice_vec);
    
    // Modificando através de slice mutável
    let slice_mut = &mut vec[2..5];
    for item in slice_mut.iter_mut() {
        *item *= 2;
    }
    println!("Vec modificado: {:?}", vec);
    
    // Demonstração de slices aninhados
    println!("\n--- Slices Aninhados ---");
    let texto = String::from("Rust é uma linguagem incrível");
    let palavras = extrair_palavras(&texto);
    println!("Texto: {}", texto);
    println!("Palavras: {:?}", palavras);
    
    // Demonstração com caracteres
    println!("\n--- Slices de Caracteres ---");
    let s_chars = "🦀🦀🦀";
    let chars_slice = &s_chars[0..4]; // 4 bytes para um emoji
    println!("String com emojis: {}", s_chars);
    println!("Slice de emoji: {}", chars_slice);
    
    // Demonstração de busca em slice
    println!("\n--- Busca em Slice ---");
    let texto_busca = "programação em Rust";
    let posicao = encontrar_palavra(&texto_busca, "Rust");
    match posicao {
        Some(pos) => {
            let slice_encontrado = &texto_busca[pos..pos + 4];
            println!("Palavra 'Rust' encontrada na posição {}: {}", pos, slice_encontrado);
        }
        None => println!("Palavra não encontrada"),
    }
}

fn primeira_palavra(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

fn somar_slice(slice: &[i32]) -> i32 {
    let mut soma = 0;
    for &item in slice {
        soma += item;
    }
    soma
}

fn extrair_palavras(s: &str) -> Vec<&str> {
    let mut palavras = Vec::new();
    let mut inicio = 0;
    
    for (i, &byte) in s.as_bytes().iter().enumerate() {
        if byte == b' ' {
            if inicio != i {
                palavras.push(&s[inicio..i]);
            }
            inicio = i + 1;
        }
    }
    
    // Adiciona a última palavra se não terminar com espaço
    if inicio < s.len() {
        palavras.push(&s[inicio..]);
    }
    
    palavras
}

fn encontrar_palavra(texto: &str, palavra: &str) -> Option<usize> {
    let bytes_texto = texto.as_bytes();
    let bytes_palavra = palavra.as_bytes();
    
    for i in 0..=bytes_texto.len().saturating_sub(bytes_palavra.len()) {
        if &bytes_texto[i..i + bytes_palavra.len()] == bytes_palavra {
            return Some(i);
        }
    }
    
    None
}

fn demonstrar_ownership_slices() {
    println!("\n--- Ownership e Slices ---");
    
    let s = String::from("hello world");
    let slice = &s[0..5];
    
    // slice é uma referência, então s ainda é válido
    println!("String: {}", s);
    println!("Slice: {}", slice);
    
    // Se tentássemos mover s, slice não seria mais válido
    // let s2 = s; // Isso tornaria slice inválido
    // println!("Slice: {}", slice); // ❌ ERRO!
}

fn demonstrar_slices_mutaveis() {
    println!("\n--- Slices Mutáveis ---");
    
    let mut array = [1, 2, 3, 4, 5];
    println!("Array original: {:?}", array);
    
    // Slice mutável
    let slice_mut = &mut array[1..4];
    
    // Modifica o slice
    for item in slice_mut.iter_mut() {
        *item *= 2;
    }
    
    println!("Array modificado: {:?}", array);
    
    // Demonstração de que não podemos ter slice mutável e imutável ao mesmo tempo
    // let slice_imut = &array[0..2]; // ❌ ERRO!
    // let slice_mut2 = &mut array[2..5]; // ❌ ERRO!
}
