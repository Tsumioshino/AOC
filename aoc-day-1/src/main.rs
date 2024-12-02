use std::collections::HashMap;

fn read_lines(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let mut lista_1: Vec<i32> = Vec::new();
    let mut lista_2: Vec<i32> = Vec::new();

    for line in std::fs::read_to_string(filename).unwrap().lines() {
        let split_line: Vec<String> = line.split("   ").map( |l| l.to_string() ).collect();
        lista_1.push(split_line[0].parse::<i32>().unwrap());
        lista_2.push(split_line[1].parse::<i32>().unwrap());
    }

    return (lista_1, lista_2);
}

fn similarity_sum(lista_1: Vec<i32>, lista_2: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for elemento in lista_1 {
        map.insert(elemento, 0);
    }
    for elemento in lista_2 {
        map.entry(elemento).and_modify( |e| *e += 1 );
    }
    let mut sum = 0;
    for (k, v) in map {
        sum += k * v
    } 
    return sum;
}

fn main() {
    // primeiro passo: parse nos inputs e converter ambos em listas
    println!("Hello, world!");
    let (mut lista_1, mut lista_2) = read_lines("resources/lista_numeracoes.txt");
    // segundo passo: ordenar as listas
    lista_1.sort();
    lista_2.sort();
    // terceiro passo: comparar os numeros e receber a diferenca entre os menores numeros;
    let mut lista_dif = Vec::new();
    for i in 0..lista_1.len() {
        lista_dif.push((lista_1[i] - lista_2[i]).abs());
    }
    // quarto e ultimo passo: somar elementos do vetor
    let mut sum = 0;
    for elemento in lista_dif {
        sum += elemento
    }
    println!("{}", sum);
    println!("{}", similarity_sum(lista_1, lista_2));

}
