use tkz::*;
use ntkz::*;
use comum::*;

pub fn analize_lexica(mut entrada: Vec<u8>) -> Result<Vec<Tokens>, i8> {
    let quebras_de_linhas = entrada
        .iter()
        .filter(|&n| *n == b'\n')
        .count();
   
    let mut tokens: Vec<Tokens> = Vec::new();

    while entrada.len() > 1 {
        if let Ok(resultado) = ntkz::irrelevantes(&entrada) {
            entrada.drain(0..resultado);
        } else if let Ok(resultado) = ntkz::comentarios_de_linha(&entrada) {
            entrada.drain(0..resultado);
        } else if let Ok(resultado) = ntkz::comentarios_de_bloco(&entrada) {
            entrada.drain(0..resultado);
        } else if let Ok(resultado) = tkz::virgula(&entrada) {
            entrada.drain(0..resultado.0);
            tokens.push(resultado.1);
        } else if let Ok(resultado) = tkz::dois_pontos(&entrada) {
            entrada.drain(0..resultado.0);
            tokens.push(resultado.1);
        } else if let Ok(resultado) = tkz::ponto_e_virgula(&entrada) {
            entrada.drain(0..resultado.0);
            tokens.push(resultado.1);
        } else if let Ok(resultado) = tkz::abre_parenteses(&entrada) {
            entrada.drain(0..resultado.0);
            tokens.push(resultado.1);
        } else if let Ok(resultado) = tkz::fecha_parenteses(&entrada) {
            entrada.drain(0..resultado.0);
            tokens.push(resultado.1);
        } else if let Ok(resultado) = tkz::set(&entrada) {
            entrada.drain(0..resultado.0);
            tokens.push(resultado.1);
        } else if let Ok(resultado) = tkz::print(&entrada) {
            entrada.drain(0..resultado.0);
            tokens.push(resultado.1);
        } else if let Ok(resultado) = tkz::scan(&entrada) {
            entrada.drain(0..resultado.0);
            tokens.push(resultado.1);
        } else if let Ok(resultado) = tkz::bloc(&entrada) {
            entrada.drain(0..resultado.0);
            tokens.push(resultado.1);
        } else if let Ok(resultado) = tkz::operador(&entrada) {
            entrada.drain(0..resultado.0);
            tokens.push(resultado.1);
        } else if let Ok(resultado) = tkz::tipo_de_variavel(&entrada) {
            entrada.drain(0..resultado.0);
            tokens.push(resultado.1);
        } else if let Ok(resultado) = tkz::id_de_variavel(&entrada) {
            entrada.drain(0..resultado.0);
            tokens.push(resultado.1);
        } else if let Ok(resultado) = tkz::id_de_bloco(&entrada) {
            entrada.drain(0..resultado.0);
            tokens.push(resultado.1);
        } else if let Ok(resultado) = tkz::abre_bloco_condicional(&entrada) {
            entrada.drain(0..resultado.0);
            tokens.push(resultado.1);
        } else if let Ok(resultado) = tkz::fecha_bloco_condicional(&entrada) {
            entrada.drain(0..resultado.0);
            tokens.push(resultado.1);
        } else if let Ok(resultado) = tkz::abre_bloco_de_codigo(&entrada) {
            entrada.drain(0..resultado.0);
            tokens.push(resultado.1);
        } else if let Ok(resultado) = tkz::fecha_bloco_de_codigo(&entrada) {
            entrada.drain(0..resultado.0);
            tokens.push(resultado.1);
        } else if let Ok(resultado) = tkz::caractere(&entrada) {
            entrada.drain(0..resultado.0);
            tokens.push(resultado.1);
        } else if let Ok(resultado) = tkz::numero(&entrada) {
            entrada.drain(0..resultado.0);
            tokens.push(resultado.1);
        } else if let Ok(resultado) = tkz::string(&entrada) {
            entrada.drain(0..resultado.0);
            tokens.push(resultado.1);
        } else {
            let quebras_de_linhas_restantes = entrada
                .iter()
                .filter(|&n| *n == b'\n')
                .count();

            println!(
                "Uma cadeia de símbolos que não pode ser reconhecida foi encontrada!\nLinha {}: {}\nSe a cadeia pertence a um comentário, a indicação de linha pode não ser precisa.",
                quebras_de_linhas - quebras_de_linhas_restantes + 1,
                String::from_utf8(entrada)
                    .expect("Cadeia UTF-8 inválida!")
                    .split_once('\n')
                    .unwrap()
                    .0
            );

            return Err(-1);
        }
    }

    return Ok(tokens);
}