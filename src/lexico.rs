use nao_tokenizadores as ntkz;
use tokenizadores as tkz;

pub fn analize_lexica(mut entrada: Vec<u8>) -> Result<String, i8> {
    let quebras_de_linhas = entrada
        .iter()
        .filter(|&n| *n == b'\n')
        .count();
   
    let mut tokens: String = String::new();

    while entrada.len() > 1 {
        if let Ok(resultado) = ntkz::irrelevantes(&entrada) {
            entrada.drain(0..resultado);
        } else if let Ok(resultado) = ntkz::comentarios_de_linha(&entrada) {
            entrada.drain(0..resultado);
        } else if let Ok(resultado) = ntkz::comentarios_de_bloco(&entrada) {
            entrada.drain(0..resultado);
        } else if let Ok(resultado) = tkz::virgula(&entrada) {
            entrada.drain(0..resultado.0);
            tokens = format!("{}\n{}", tokens, resultado.1);
        } else if let Ok(resultado) = tkz::dois_pontos(&entrada) {
            entrada.drain(0..resultado.0);
            tokens = format!("{}\n{}", tokens, resultado.1);
        } else if let Ok(resultado) = tkz::ponto_e_virgula(&entrada) {
            entrada.drain(0..resultado.0);
            tokens = format!("{}\n{}", tokens, resultado.1);
        } else if let Ok(resultado) = tkz::abre_parenteses(&entrada) {
            entrada.drain(0..resultado.0);
            tokens = format!("{}\n{}", tokens, resultado.1);
        } else if let Ok(resultado) = tkz::fecha_parenteses(&entrada) {
            entrada.drain(0..resultado.0);
            tokens = format!("{}\n{}", tokens, resultado.1);
        } else if let Ok(resultado) = tkz::set(&entrada) {
            entrada.drain(0..resultado.0);
            tokens = format!("{}\n{}", tokens, resultado.1);
        } else if let Ok(resultado) = tkz::print(&entrada) {
            entrada.drain(0..resultado.0);
            tokens = format!("{}\n{}", tokens, resultado.1);
        } else if let Ok(resultado) = tkz::scan(&entrada) {
            entrada.drain(0..resultado.0);
            tokens = format!("{}\n{}", tokens, resultado.1);
        } else if let Ok(resultado) = tkz::bloc(&entrada) {
            entrada.drain(0..resultado.0);
            tokens = format!("{}\n{}", tokens, resultado.1);
        } else if let Ok(resultado) = tkz::operador(&entrada) {
            entrada.drain(0..resultado.0);
            tokens = format!("{}\n{}", tokens, resultado.1);
        } else if let Ok(resultado) = tkz::tipo_de_variavel(&entrada) {
            entrada.drain(0..resultado.0);
            tokens = format!("{}\n{}", tokens, resultado.1);
        } else if let Ok(resultado) = tkz::id_de_variavel(&entrada) {
            entrada.drain(0..resultado.0);
            tokens = format!("{}\n{}", tokens, resultado.1);
        } else if let Ok(resultado) = tkz::id_de_bloco(&entrada) {
            entrada.drain(0..resultado.0);
            tokens = format!("{}\n{}", tokens, resultado.1);
        } else if let Ok(resultado) = tkz::abre_bloco_condicional(&entrada) {
            entrada.drain(0..resultado.0);
            tokens = format!("{}\n{}", tokens, resultado.1);
        } else if let Ok(resultado) = tkz::fecha_bloco_condicional(&entrada) {
            entrada.drain(0..resultado.0);
            tokens = format!("{}\n{}", tokens, resultado.1);
        } else if let Ok(resultado) = tkz::abre_bloco_de_codigo(&entrada) {
            entrada.drain(0..resultado.0);
            tokens = format!("{}\n{}", tokens, resultado.1);
        } else if let Ok(resultado) = tkz::fecha_bloco_de_codigo(&entrada) {
            entrada.drain(0..resultado.0);
            tokens = format!("{}\n{}", tokens, resultado.1);
        } else if let Ok(resultado) = tkz::caractere(&entrada) {
            entrada.drain(0..resultado.0);
            tokens = format!("{}\n{}", tokens, resultado.1);
        } else if let Ok(resultado) = tkz::numero(&entrada) {
            entrada.drain(0..resultado.0);
            tokens = format!("{}\n{}", tokens, resultado.1);
        } else if let Ok(resultado) = tkz::string(&entrada) {
            entrada.drain(0..resultado.0);
            tokens = format!("{}\n{}", tokens, resultado.1);
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

mod nao_tokenizadores {
    use pom::parser::*;

    pub fn irrelevantes(entrada: &[u8]) -> Result<usize, i8> {
        let analizador = one_of(b" \n\t").repeat(1..);
        
        match analizador.parse(entrada) {
            Ok(saida) => {
                return Ok(saida.len());
            }

            Err(_e) => {
                return Err(-1);
            }
        };
    }

    pub fn comentarios_de_linha(entrada: &[u8]) -> Result<usize, i8> {
        let simbolos = b"\t 1234567890qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM";

        let analizador = seq(b"//") + one_of(simbolos.as_ref()).repeat(0..) + sym(b'\n');

        match analizador.parse(entrada) {
            Ok(saida) => {
                let mut resultado: Vec<u8> = Vec::new();

                resultado.push(saida.0 .0[0]);
                resultado.push(saida.0 .0[1]);

                for caractere in saida.0 .1 {
                    resultado.push(caractere.into());
                }

                resultado.push(saida.1);

                return Ok(resultado.len());
            }

            Err(_e) => {
                return Err(-1);
            }
        };
    }

    pub fn comentarios_de_bloco(entrada: &[u8]) -> Result<usize, i8> {
        let simbolos = b"\n\t 1234567890qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM";

        let analizador = seq(b"!--") + one_of(simbolos.as_ref()).repeat(0..) + seq(b"--!");

        match analizador.parse(entrada) {
            Ok(saida) => {
                let mut resultado: Vec<u8> = Vec::new();

                resultado.push(saida.0 .0[0]);
                resultado.push(saida.0 .0[1]);
                resultado.push(saida.0 .0[2]);

                for caractere in saida.0 .1 {
                    resultado.push(caractere);
                }

                resultado.push(saida.1[0]);
                resultado.push(saida.1[1]);
                resultado.push(saida.1[2]);

                return Ok(resultado.len());
            }

            Err(_e) => {
                return Err(-1);
            }
        };
    }
}

mod tokenizadores {
    use pom::parser::*;
    use std::str::FromStr;

    pub fn bloc(entrada: &[u8]) -> Result<(usize, String), i8> {
        let analizador = seq(b"BLOC");

        match analizador.parse(entrada) {
            Ok(saida) => {
                let mut resultado: Vec<u8> = Vec::new();

                for caractere in saida {
                    resultado.push(*caractere);
                }

                return Ok((resultado.len(), String::from_str("[bloc, ]").unwrap()));
            }

            Err(_e) => {
                return Err(-1);
            }
        };
    }

    pub fn set(entrada: &[u8]) -> Result<(usize, String), i8> {
        let analizador = seq(b"SET");

        match analizador.parse(entrada) {
            Ok(saida) => {
                let mut resultado: Vec<u8> = Vec::new();

                for caractere in saida {
                    resultado.push(*caractere);
                }

                return Ok((resultado.len(), String::from_str("[set, ]").unwrap()));
            }

            Err(_e) => {
                return Err(-1);
            }
        };
    }

    pub fn print(entrada: &[u8]) -> Result<(usize, String), i8> {
        let analizador = seq(b"PRINT");

        match analizador.parse(entrada) {
            Ok(saida) => {
                let mut resultado: Vec<u8> = Vec::new();

                for caractere in saida {
                    resultado.push(*caractere);
                }

                return Ok((resultado.len(), String::from_str("[print, ]").unwrap()));
            }

            Err(_e) => {
                return Err(-1);
            }
        };
    }

    pub fn scan(entrada: &[u8]) -> Result<(usize, String), i8> {
        let analizador = seq(b"SCAN");

        match analizador.parse(entrada) {
            Ok(saida) => {
                let mut resultado: Vec<u8> = Vec::new();

                for caractere in saida {
                    resultado.push(*caractere);
                }

                return Ok((resultado.len(), String::from_str("[scan, ]").unwrap()));
            }

            Err(_e) => {
                return Err(-1);
            }
        };
    }

    pub fn operador(entrada: &[u8]) -> Result<(usize, String), i8> {
        let analizador = seq(b"DIVR")
            | seq(b"NOT")
            | seq(b"AND")
            | seq(b"ADD")
            | seq(b"SUB")
            | seq(b"MUL")
            | seq(b"DIV")
            | seq(b"OR")
            | seq(b"AE")
            | seq(b"BE")
            | seq(b"A")
            | seq(b"E")
            | seq(b"B");

        match analizador.parse(entrada) {
            Ok(saida) => {
                let mut resultado: Vec<u8> = Vec::new();

                for caractere in saida {
                    resultado.push(*caractere);
                }

                return Ok((
                    resultado.len(),
                    format!("[operador, {}]", String::from_utf8(resultado).unwrap()),
                ));
            }

            Err(_e) => {
                return Err(-1);
            }
        };
    }

    pub fn abre_bloco_condicional(entrada: &[u8]) -> Result<(usize, String), i8> {
        let analizador = sym(b'#') + (seq(b"INZ") | seq(b"WNZ") | seq(b"RUI")) + sym(b':');

        match analizador.parse(entrada) {
            Ok(saida) => {
                let mut resultado: Vec<u8> = Vec::new();

                resultado.push(saida.0 .0);

                for caractere in saida.0 .1 {
                    resultado.push(*caractere);
                }

                resultado.push(saida.1);

                let tamanho = resultado.len();

                resultado.drain(0..1);
                resultado.drain((resultado.len() - 1)..(resultado.len()));

                return Ok((
                    tamanho,
                    format!(
                        "[abre_bloco_condicional, {}]",
                        String::from_utf8(resultado).unwrap()
                    ),
                ));
            }

            Err(_e) => {
                return Err(-1);
            }
        };
    }

    pub fn fecha_bloco_condicional(entrada: &[u8]) -> Result<(usize, String), i8> {
        let analizador = sym(b'#') + (seq(b"INZ") | seq(b"WNZ") | seq(b"RUI")) + sym(b';');

        match analizador.parse(entrada) {
            Ok(saida) => {
                let mut resultado: Vec<u8> = Vec::new();

                resultado.push(saida.0 .0);

                for caractere in saida.0 .1 {
                    resultado.push(*caractere);
                }

                resultado.push(saida.1);

                let tamanho = resultado.len();

                resultado.drain(0..1);
                resultado.drain((resultado.len() - 1)..(resultado.len()));

                return Ok((
                    tamanho,
                    format!(
                        "[fecha_bloco_condicional, {}]",
                        String::from_utf8(resultado).unwrap()
                    ),
                ));
            }

            Err(_e) => {
                return Err(-1);
            }
        };
    }

    pub fn abre_bloco_de_codigo(entrada: &[u8]) -> Result<(usize, String), i8> {
        let simbolos_de_inicio = b"QWERTYUIOPASDFGHJKLZXCVBNM";
        let simbolos = b"_1234567890QWERTYUIOPASDFGHJKLZXCVBNM";

        let analizador = sym(b'#')
            + one_of(simbolos_de_inicio.as_ref())
            + one_of(simbolos.as_ref()).repeat(0..)
            + sym(b':');

        match analizador.parse(entrada) {
            Ok(saida) => {
                let mut resultado: Vec<u8> = Vec::new();

                resultado.push(saida.0 .0 .0);
                resultado.push(saida.0 .0 .1);

                for caractere in saida.0 .1 {
                    resultado.push(caractere);
                }

                resultado.push(saida.1);

                let tamanho = resultado.len();

                resultado.drain(0..1);
                resultado.drain((resultado.len() - 1)..(resultado.len()));

                return Ok((
                    tamanho,
                    format!(
                        "[abre_bloco_de_codigo, {}]",
                        String::from_utf8(resultado).unwrap()
                    ),
                ));
            }

            Err(_e) => {
                return Err(-1);
            }
        };
    }

    pub fn fecha_bloco_de_codigo(entrada: &[u8]) -> Result<(usize, String), i8> {
        let simbolos_de_inicio = b"QWERTYUIOPASDFGHJKLZXCVBNM";
        let simbolos = b"_1234567890QWERTYUIOPASDFGHJKLZXCVBNM";

        let analizador = sym(b'#')
            + one_of(simbolos_de_inicio.as_ref())
            + one_of(simbolos.as_ref()).repeat(0..)
            + sym(b';');

        match analizador.parse(entrada) {
            Ok(saida) => {
                let mut resultado: Vec<u8> = Vec::new();

                resultado.push(saida.0 .0 .0);
                resultado.push(saida.0 .0 .1);

                for caractere in saida.0 .1 {
                    resultado.push(caractere);
                }

                resultado.push(saida.1);

                let tamanho = resultado.len();

                resultado.drain(0..1);
                resultado.drain((resultado.len() - 1)..(resultado.len()));

                return Ok((
                    tamanho,
                    format!(
                        "[fecha_bloco_de_codigo, {}]",
                        String::from_utf8(resultado).unwrap()
                    ),
                ));
            }

            Err(_e) => {
                return Err(-1);
            }
        };
    }

    pub fn tipo_de_variavel(entrada: &[u8]) -> Result<(usize, String), i8> {
        let analizador = seq(b"CHR")
            | seq(b"STR")
            | seq(b"INT8")
            | seq(b"INT16")
            | seq(b"INT32")
            | seq(b"INT64")
            | seq(b"UINT8")
            | seq(b"UINT16")
            | seq(b"UINT32")
            | seq(b"UINT64");

        match analizador.parse(entrada) {
            Ok(saida) => {
                let mut resultado: Vec<u8> = Vec::new();

                for caractere in saida {
                    resultado.push(*caractere);
                }

                return Ok((
                    resultado.len(),
                    format!(
                        "[tipo_de_variavel, {}]",
                        String::from_utf8(resultado).unwrap()
                    ),
                ));
            }

            Err(_e) => {
                return Err(-1);
            }
        };
    }

    pub fn id_de_variavel(entrada: &[u8]) -> Result<(usize, String), i8> {
        let simbolos_de_inicio = b"qwertyuiopasdfghjklzxcvbnm";
        let simbolos = b"_1234567890qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM";

        let analizador =
            one_of(simbolos_de_inicio.as_ref()) + one_of(simbolos.as_ref()).repeat(0..);

        match analizador.parse(entrada) {
            Ok(saida) => {
                let mut resultado: Vec<u8> = Vec::new();

                resultado.push(saida.0);

                for caractere in saida.1 {
                    resultado.push(caractere);
                }

                return Ok((
                    resultado.len(),
                    format!(
                        "[id_de_variavel, {}]",
                        String::from_utf8(resultado).unwrap()
                    ),
                ));
            }

            Err(_e) => {
                return Err(-1);
            }
        };
    }

    pub fn id_de_bloco(entrada: &[u8]) -> Result<(usize, String), i8> {
        let simbolos_de_inicio = b"QWERTYUIOPASDFGHJKLZXCVBNM";
        let simbolos = b"_1234567890QWERTYUIOPASDFGHJKLZXCVBNM";

        let analizador =
            one_of(simbolos_de_inicio.as_ref()) + one_of(simbolos.as_ref()).repeat(0..);

        match analizador.parse(entrada) {
            Ok(saida) => {
                let mut resultado: Vec<u8> = Vec::new();

                resultado.push(saida.0);

                for caractere in saida.1 {
                    resultado.push(caractere);
                }

                return Ok((
                    resultado.len(),
                    format!("[id_de_bloco, {}]", String::from_utf8(resultado).unwrap()),
                ));
            }

            Err(_e) => {
                return Err(-1);
            }
        };
    }

    pub fn virgula(entrada: &[u8]) -> Result<(usize, String), i8> {
        let analizador = sym(b',');

        match analizador.parse(entrada) {
            Ok(_saida) => {
                return Ok((1, String::from_str("[virgula, ]").unwrap()));
            }

            Err(_e) => {
                return Err(-1);
            }
        };
    }

    pub fn ponto_e_virgula(entrada: &[u8]) -> Result<(usize, String), i8> {
        let analizador = sym(b';');

        match analizador.parse(entrada) {
            Ok(_saida) => {
                return Ok((1, String::from_str("[ponto_e_virgula, ]").unwrap()));
            }

            Err(_e) => {
                return Err(-1);
            }
        };
    }

    pub fn abre_parenteses(entrada: &[u8]) -> Result<(usize, String), i8> {
        let analizador = sym(b'(');

        match analizador.parse(entrada) {
            Ok(_saida) => {
                return Ok((1, String::from_str("[abre_parenteses, ]").unwrap()));
            }

            Err(_e) => {
                return Err(-1);
            }
        };
    }

    pub fn fecha_parenteses(entrada: &[u8]) -> Result<(usize, String), i8> {
        let analizador = sym(b')');

        match analizador.parse(entrada) {
            Ok(_saida) => {
                return Ok((1, String::from_str("[fecha_parenteses, ]").unwrap()));
            }

            Err(_e) => {
                return Err(-1);
            }
        };
    }

    pub fn dois_pontos(entrada: &[u8]) -> Result<(usize, String), i8> {
        let analizador = sym(b':');

        match analizador.parse(entrada) {
            Ok(_saida) => {
                return Ok((1, String::from_str("[dois_pontos, ]").unwrap()));
            }

            Err(_e) => {
                return Err(-1);
            }
        };
    }

    pub fn numero(entrada: &[u8]) -> Result<(usize, String), i8> {
        let simbolos = b"1234567890";

        let analizador = (sym(b'+') | sym(b'-') | one_of(simbolos.as_ref()))
            + one_of(simbolos.as_ref()).repeat(0..);

        match analizador.parse(entrada) {
            Ok(saida) => {
                let mut resultado: Vec<u8> = Vec::new();

                resultado.push(saida.0);

                for caractere in saida.1 {
                    resultado.push(caractere);
                }

                return Ok((
                    resultado.len(),
                    format!("[numero, {}]", String::from_utf8(resultado).unwrap()),
                ));
            }

            Err(_e) => {
                return Err(-1);
            }
        };
    }

    pub fn string(entrada: &[u8]) -> Result<(usize, String), i8> {
        let simbolos = b"\n\t\0 1234567890qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM";

        let analizador = sym(b'"') + one_of(simbolos.as_ref()).repeat(0..) + sym(b'"');

        match analizador.parse(entrada) {
            Ok(saida) => {
                let mut resultado: Vec<u8> = Vec::new();

                resultado.push(saida.0 .0);

                for caractere in saida.0 .1 {
                    resultado.push(caractere);
                }

                resultado.push(saida.1);

                return Ok((
                    resultado.len(),
                    format!("[string, {}]", String::from_utf8(resultado).unwrap()),
                ));
            }

            Err(_e) => {
                return Err(-1);
            }
        };
    }

    pub fn caractere(entrada: &[u8]) -> Result<(usize, String), i8> {
        let simbolos = b"\n\t\0 1234567890qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM";

        let analizador = sym(b'\'') + one_of(simbolos.as_ref()) + sym(b'\'');

        match analizador.parse(entrada) {
            Ok(saida) => {
                let mut resultado: Vec<u8> = Vec::new();

                resultado.push(saida.0 .0);
                resultado.push(saida.0 .1);
                resultado.push(saida.1);

                return Ok((
                    resultado.len(),
                    format!("[caractere, {}]", String::from_utf8(resultado).unwrap()),
                ));
            }

            Err(_e) => {
                return Err(-1);
            }
        };
    }
}
