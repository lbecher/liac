use std::env;
use std::fs::File;
use std::io::prelude::*;

mod comum;
use crate::comum::Tokens;

mod lexico;
use lexico::analize_lexica;

mod sintatico;
use sintatico::Sintatico;

fn main() {
    // obtém argumentos do terminal
    let argumentos: Vec<String> = env::args().collect();

    // abre arquivo .lia
    let mut arquivo = File::open(argumentos[1].clone())
        .expect("Não foi possível abrir o arquivo de entrada!");

    let mut entrada: Vec<u8> = Vec::new();

    arquivo.read_to_end(&mut entrada)
        .expect("Não foi possível ler o arquivo de entrada!");

    let entrada_string: String = String::from_utf8(entrada)
        .expect("Não foi possível gerar uma string a partir do arquivo!");

    // inicia analizador léxico
    if let Ok(tokens) = analize_lexica(entrada_string.as_bytes().to_vec()) {
        // prepara cópia do vetor para o analizador sintático
        let mut tokens_copia = tokens.to_vec();
        tokens_copia.push(Tokens::Fim);

        // inicia analizador sintático
        let mut sintatico = Sintatico::inicializar(&tokens_copia);
        sintatico.analisar();
    }
}
