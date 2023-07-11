use std::env;
use std::fs::File;
use std::io::prelude::*;

mod lexico;
mod sintatico;
mod semantico;
mod llvm;

use liac::Tokens;
use lexico::analize_lexica;
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

    // inicia analizador léxico
    if let Ok(tokens) = analize_lexica(entrada) {
        // prepara cópia do vetor para o analizador sintático
        let mut tokens_copia = tokens.to_vec();
        tokens_copia.push(Tokens::Fim);

        // inicia analizador sintático
        let mut sintatico = Sintatico::inicializar(&tokens_copia);
        sintatico.analisar();
    }
}
