fn main() {
    println!("=== Exemplo: Vec - Lista Dinâmica ===");
    
    // Criando um Vec
    let mut numeros = Vec::new();
    
    // Adicionando elementos
    numeros.push(1);
    numeros.push(2);
    numeros.push(3);
    println!("Vec inicial: {:?}", numeros);
    
    // Criando Vec com valores iniciais
    let mut frutas = vec!["maçã", "banana", "laranja"];
    println!("Frutas: {:?}", frutas);
    
    // Acessando elementos
    println!("Primeira fruta: {}", frutas[0]);
    println!("Última fruta: {:?}", frutas.last());
    
    // Modificando elementos
    frutas[1] = "uva";
    println!("Frutas modificadas: {:?}", frutas);
    
    // Removendo elementos
    let removido = frutas.pop();
    println!("Removido: {:?}", removido);
    println!("Frutas após remoção: {:?}", frutas);
    
    // Iterando sobre Vec
    println!("\nIterando sobre frutas:");
    for (indice, fruta) in frutas.iter().enumerate() {
        println!("  {}: {}", indice, fruta);
    }
    
    // Operações com Vec
    let mut numeros = vec![1, 2, 3, 4, 5];
    
    // Filtrar números pares
    let pares: Vec<i32> = numeros.iter()
        .filter(|&x| x % 2 == 0)
        .cloned()
        .collect();
    println!("Números pares: {:?}", pares);
    
    // Mapear para quadrados
    let quadrados: Vec<i32> = numeros.iter()
        .map(|x| x * x)
        .collect();
    println!("Quadrados: {:?}", quadrados);
    
    // Soma dos elementos
    let soma: i32 = numeros.iter().sum();
    println!("Soma: {}", soma);
    
    // Encontrar elemento
    let encontrado = numeros.iter().find(|&&x| x > 3);
    println!("Primeiro elemento > 3: {:?}", encontrado);
    
    // Capacidade e tamanho
    println!("Tamanho: {}", numeros.len());
    println!("Capacidade: {}", numeros.capacity());
    
    // Redimensionar
    numeros.resize(10, 0);
    println!("Após resize: {:?}", numeros);
    
    // Limpar
    numeros.clear();
    println!("Após clear: {:?}", numeros);
    
    // Demonstração de diferentes tipos
    println!("\n--- Vec com diferentes tipos ---");
    
    // Vec de strings
    let mut palavras = vec![String::from("hello"), String::from("world")];
    palavras.push(String::from("rust"));
    println!("Palavras: {:?}", palavras);
    
    // Vec de structs
    #[derive(Debug)]
    struct Pessoa {
        nome: String,
        idade: u32,
    }
    
    let mut pessoas = vec![
        Pessoa { nome: String::from("Alice"), idade: 25 },
        Pessoa { nome: String::from("Bob"), idade: 30 },
    ];
    pessoas.push(Pessoa { nome: String::from("Charlie"), idade: 35 });
    println!("Pessoas: {:?}", pessoas);
    
    // Vec de tuplas
    let mut coordenadas = vec![(0, 0), (1, 1), (2, 2)];
    coordenadas.push((3, 3));
    println!("Coordenadas: {:?}", coordenadas);
    
    // Operações avançadas com Vec
    println!("\n--- Operações Avançadas ---");
    
    let mut dados = vec![5, 2, 8, 1, 9, 3, 7, 4, 6];
    
    // Ordenar
    dados.sort();
    println!("Ordenado: {:?}", dados);
    
    // Ordenar em ordem decrescente
    dados.sort_by(|a, b| b.cmp(a));
    println!("Ordenado decrescente: {:?}", dados);
    
    // Inverter
    dados.reverse();
    println!("Invertido: {:?}", dados);
    
    // Remover duplicatas
    let mut com_duplicatas = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
    com_duplicatas.sort();
    com_duplicatas.dedup();
    println!("Sem duplicatas: {:?}", com_duplicatas);
    
    // Dividir em chunks
    let chunks: Vec<&[i32]> = dados.chunks(3).collect();
    println!("Chunks de 3: {:?}", chunks);
    
    // Windows (sliding window)
    let windows: Vec<&[i32]> = dados.windows(2).collect();
    println!("Windows de 2: {:?}", windows);
    
    // Split em partes
    let mut texto = vec![1, 2, 0, 3, 4, 0, 5, 6];
    let partes: Vec<Vec<i32>> = texto.split(|&x| x == 0)
        .map(|slice| slice.to_vec())
        .collect();
    println!("Partes separadas por 0: {:?}", partes);
    
    // Concatenar Vecs
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    let combinado = [vec1, vec2].concat();
    println!("Combinado: {:?}", combinado);
    
    // Extender com outro Vec
    let mut base = vec![1, 2, 3];
    let adicionar = vec![4, 5, 6];
    base.extend(adicionar);
    println!("Estendido: {:?}", base);
}
