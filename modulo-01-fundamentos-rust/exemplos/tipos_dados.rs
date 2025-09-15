fn main() {
    println!("=== Exemplo: Tipos de Dados Primitivos ===");
    
    // INTEIROS COM SINAL (podem ser negativos)
    let inteiro_8: i8 = -128;      // -128 a 127
    let inteiro_16: i16 = -32768;  // -32,768 a 32,767
    let inteiro_32: i32 = -1000;   // -2,147,483,648 a 2,147,483,647
    let inteiro_64: i64 = -999999; // -9,223,372,036,854,775,808 a 9,223,372,036,854,775,807
    let inteiro_128: i128 = -1000000000000000000; // Muito grande
    let inteiro_isize: isize = -42; // Depende da arquitetura (32 ou 64 bits)
    
    println!("Inteiros com sinal:");
    println!("i8: {}, i16: {}, i32: {}, i64: {}, i128: {}, isize: {}", 
             inteiro_8, inteiro_16, inteiro_32, inteiro_64, inteiro_128, inteiro_isize);
    
    // INTEIROS SEM SINAL (apenas positivos)
    let sem_sinal_8: u8 = 255;     // 0 a 255
    let sem_sinal_16: u16 = 65535; // 0 a 65,535
    let sem_sinal_32: u32 = 42;    // 0 a 4,294,967,295
    let sem_sinal_64: u64 = 1000;  // 0 a 18,446,744,073,709,551,615
    let sem_sinal_128: u128 = 5000000000000000000; // Muito grande
    let sem_sinal_usize: usize = 100; // Depende da arquitetura
    
    println!("\nInteiros sem sinal:");
    println!("u8: {}, u16: {}, u32: {}, u64: {}, u128: {}, usize: {}", 
             sem_sinal_8, sem_sinal_16, sem_sinal_32, sem_sinal_64, sem_sinal_128, sem_sinal_usize);
    
    // PONTO FLUTUANTE
    let flutuante_32: f32 = 3.14159;
    let flutuante_64: f64 = 2.718281828459045;
    
    println!("\nPonto flutuante:");
    println!("f32: {}, f64: {}", flutuante_32, flutuante_64);
    
    // BOOLEANO
    let verdadeiro: bool = true;
    let falso: bool = false;
    
    println!("\nBooleanos:");
    println!("true: {}, false: {}", verdadeiro, falso);
    
    // CARACTERE (Unicode, 4 bytes)
    let caractere_ascii: char = 'A';
    let caractere_acento: char = 'á';
    let caractere_emoji: char = '🦀';
    let caractere_chinês: char = '中';
    
    println!("\nCaracteres:");
    println!("ASCII: {}, Acento: {}, Emoji: {}, Chinês: {}", 
             caractere_ascii, caractere_acento, caractere_emoji, caractere_chinês);
    
    // TUPLA (agrupa valores de tipos diferentes)
    let tupla: (i32, f64, bool) = (42, 3.14, true);
    let tupla_nomeada = (idade: 25, altura: 1.75, casado: false);
    
    println!("\nTuplas:");
    println!("Tupla simples: {:?}", tupla);
    println!("Tupla nomeada: idade={}, altura={}, casado={}", 
             tupla_nomeada.0, tupla_nomeada.1, tupla_nomeada.2);
    
    // Desestruturação de tupla
    let (x, y, z) = tupla;
    println!("Desestruturação: x={}, y={}, z={}", x, y, z);
    
    // ARRAY (elementos do mesmo tipo, tamanho fixo)
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let array_zeros = [0; 10]; // 10 zeros
    
    println!("\nArrays:");
    println!("Array: {:?}", array);
    println!("Array de zeros: {:?}", array_zeros);
    println!("Primeiro elemento: {}", array[0]);
    println!("Último elemento: {}", array[array.len() - 1]);
    
    // Demonstração de inferência de tipo
    let numero = 42;        // i32 por padrão
    let decimal = 3.14;     // f64 por padrão
    let texto = "Olá";      // &str
    
    println!("\nInferência de tipos:");
    println!("numero: {}, decimal: {}, texto: {}", numero, decimal, texto);
    
    // Literais numéricos com separadores
    let milhao = 1_000_000;
    let hexa = 0xff;
    let octal = 0o77;
    let binario = 0b1111_0000;
    
    println!("\nLiterais numéricos:");
    println!("Milhão: {}, Hexa: {}, Octal: {}, Binário: {}", 
             milhao, hexa, octal, binario);
}
