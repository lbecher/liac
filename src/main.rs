use std::env;
use std::fs::File;
use std::io::prelude::*;

mod lexico;
use lexico::analize_lexica;

fn main() {
    // obtém argumentos do terminal
    let argumentos: Vec<String> = env::args().collect();

    // abre arquivo .lia
    let mut arquivo = File::open(argumentos.get(1).unwrap())
        .expect("Não foi possível abrir o arquivo de entrada!");

    let mut entrada: Vec<u8> = Vec::new();

    arquivo.read_to_end(&mut entrada)
        .expect("Não foi possível ler o arquivo de entrada!");

    let mut entrada_string: String = String::from_utf8(entrada)
        .expect("Não foi possível gerar uma string a partir do arquivo!");

    // trata símbolos especiais
    entrada_string = entrada_string.replace("\\n", "\n");
    entrada_string = entrada_string.replace("\\t", "\t");
    entrada_string = entrada_string.replace("\\0", "\0");

    // inicia analizador léxico
    let tokens = analize_lexica(entrada_string.as_bytes().to_vec());

    println!("{}", tokens);
}
