use debug_print::debug_println;

use liac::*;

pub fn analize_lexica(mut entrada: Vec<u8>) -> Result<Vec<Tokens>, ()> {
    let mut tokens: Vec<Tokens> = Vec::new();

    let mut quebras_de_linha: usize = 0;
    let mut leitura_comentario_de_bloco = false;
    let mut leitura_comentario_de_linha = false;

    println!("---------------------------------------\nIniciando análise léxica...\n-------------\n");

    while entrada.len() > 1 {
        if leitura_comentario_de_bloco {
            if let Ok(resultado) = tkz::quebra_de_linha(&entrada) {
                entrada.drain(0..resultado.0);
                tokens.push(resultado.1);
                quebras_de_linha += 1;
            } else if entrada.len() > 2 && entrada[0..3] == vec![b'-', b'-', b'!'] {
                entrada.remove(0);
                entrada.remove(0);
                entrada.remove(0);
                leitura_comentario_de_bloco = false;
            } else if entrada.len() > 2 {
                entrada.remove(0);
            } else {
                println!("-------------\nERRO SINTÁTICO: O comentário de bloco não foi fechado!\nLinha {}.\n-------------\n", quebras_de_linha + 1);
                
                return Err(());
            }
        }

        else if leitura_comentario_de_linha {
            if let Ok(resultado) = tkz::quebra_de_linha(&entrada) {
                entrada.drain(0..resultado.0);
                tokens.push(resultado.1);
                quebras_de_linha += 1;
                leitura_comentario_de_linha = false;
            } else {
                entrada.remove(0);
            }
        }

        else {
            if let Ok(resultado) = ntkz::irrelevantes(&entrada) {
                entrada.drain(0..resultado);
            } else if let Ok(resultado) = ntkz::abre_comentario_de_linha(&entrada) {
                entrada.drain(0..resultado);
                leitura_comentario_de_linha = true;
            } else if let Ok(resultado) = ntkz::abre_comentario_de_bloco(&entrada) {
                entrada.drain(0..resultado);
                leitura_comentario_de_bloco = true;
            } else if let Ok(resultado) = tkz::quebra_de_linha(&entrada) {
                entrada.drain(0..resultado.0);
                tokens.push(resultado.1);
                quebras_de_linha += 1;
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
            } else if let Ok(resultado) = tkz::retorno(&entrada) {
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
            } else if let Ok(resultado) = tkz::abre_bloco_inz(&entrada) {
                entrada.drain(0..resultado.0);
                tokens.push(resultado.1);
            } else if let Ok(resultado) = tkz::fecha_bloco_inz(&entrada) {
                entrada.drain(0..resultado.0);
                tokens.push(resultado.1);
            } else if let Ok(resultado) = tkz::abre_bloco_wnz(&entrada) {
                entrada.drain(0..resultado.0);
                tokens.push(resultado.1);
            } else if let Ok(resultado) = tkz::fecha_bloco_wnz(&entrada) {
                entrada.drain(0..resultado.0);
                tokens.push(resultado.1);
            } else if let Ok(resultado) = tkz::abre_bloco_rui(&entrada) {
                entrada.drain(0..resultado.0);
                tokens.push(resultado.1);
            } else if let Ok(resultado) = tkz::fecha_bloco_rui(&entrada) {
                entrada.drain(0..resultado.0);
                tokens.push(resultado.1);
            } else if let Ok(resultado) = tkz::abre_bloco_data(&entrada) {
                entrada.drain(0..resultado.0);
                tokens.push(resultado.1);
            } else if let Ok(resultado) = tkz::fecha_bloco_data(&entrada) {
                entrada.drain(0..resultado.0);
                tokens.push(resultado.1);
            } else if let Ok(resultado) = tkz::abre_bloco_main(&entrada) {
                entrada.drain(0..resultado.0);
                tokens.push(resultado.1);
            } else if let Ok(resultado) = tkz::fecha_bloco_main(&entrada) {
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
                println!("-------------\nERRO LÉXICO: Sequência de caracteres inesperada!\nLinha {}.\n-------------\n", quebras_de_linha + 1);
                return Err(());
            }
        }
    }

    debug_println!("{:?}", tokens);

    println!("-------------\nAnálise léxica terminou sem erros.\n---------------------------------------\n");

    return Ok(tokens);
}

mod ntkz {
    use pom::parser::*;

    pub fn irrelevantes(entrada: &[u8]) -> Result<usize, ()> {
        let analizador = one_of(b" \t").repeat(1..);
        
        match analizador.parse(entrada) {
            Ok(saida) => {
                return Ok(saida.len());
            }
    
            Err(_e) => {
                return Err(());
            }
        };
    }
    
    pub fn abre_comentario_de_linha(entrada: &[u8]) -> Result<usize, ()> {
        let analizador = seq(b"//");
    
        match analizador.parse(entrada) {
            Ok(saida) => {    
                return Ok(saida.len());
            }
    
            Err(_e) => {
                return Err(());
            }
        };
    }
    
    pub fn abre_comentario_de_bloco(entrada: &[u8]) -> Result<usize, ()> {
        let analizador = seq(b"!--");
    
        match analizador.parse(entrada) {
            Ok(saida) => {
                return Ok(saida.len());
            }
    
            Err(_e) => {
                return Err(());
            }
        };
    }
}

mod tkz {

    use pom::parser::*;

    use crate::lexico::Tokens;

    pub fn quebra_de_linha(entrada: &[u8]) -> Result<(usize, Tokens), ()> {
        let analizador = seq(b"\n");

        match analizador.parse(entrada) {
            Ok(saida) => {
                let mut resultado: Vec<u8> = Vec::new();

                for caractere in saida {
                    resultado.push(*caractere);
                }

                return Ok((resultado.len(), Tokens::QuebraDeLinha));
            }

            Err(_e) => {
                return Err(());
            }
        };
    }

    pub fn retorno(entrada: &[u8]) -> Result<(usize, Tokens), ()> {
        let analizador = seq(b"RETURN");

        match analizador.parse(entrada) {
            Ok(saida) => {
                let mut resultado: Vec<u8> = Vec::new();

                for caractere in saida {
                    resultado.push(*caractere);
                }

                return Ok((resultado.len(), Tokens::Return));
            }

            Err(_e) => {
                return Err(());
            }
        };
    }

    pub fn bloc(entrada: &[u8]) -> Result<(usize, Tokens), ()> {
        let analizador = seq(b"BLOC");

        match analizador.parse(entrada) {
            Ok(saida) => {
                let mut resultado: Vec<u8> = Vec::new();

                for caractere in saida {
                    resultado.push(*caractere);
                }

                return Ok((resultado.len(), Tokens::Bloc));
            }

            Err(_e) => {
                return Err(());
            }
        };
    }

    pub fn set(entrada: &[u8]) -> Result<(usize, Tokens), ()> {
        let analizador = seq(b"SET");

        match analizador.parse(entrada) {
            Ok(saida) => {
                let mut resultado: Vec<u8> = Vec::new();

                for caractere in saida {
                    resultado.push(*caractere);
                }

                return Ok((resultado.len(), Tokens::Set));
            }

            Err(_e) => {
                return Err(());
            }
        };
    }

    pub fn print(entrada: &[u8]) -> Result<(usize, Tokens), ()> {
        let analizador = seq(b"PRINT");

        match analizador.parse(entrada) {
            Ok(saida) => {
                let mut resultado: Vec<u8> = Vec::new();

                for caractere in saida {
                    resultado.push(*caractere);
                }

                return Ok((resultado.len(), Tokens::Print));
            }

            Err(_e) => {
                return Err(());
            }
        };
    }

    pub fn scan(entrada: &[u8]) -> Result<(usize, Tokens), ()> {
        let analizador = seq(b"SCAN");

        match analizador.parse(entrada) {
            Ok(saida) => {
                let mut resultado: Vec<u8> = Vec::new();

                for caractere in saida {
                    resultado.push(*caractere);
                }

                return Ok((resultado.len(), Tokens::Scan));
            }

            Err(_e) => {
                return Err(());
            }
        };
    }

    pub fn abre_bloco_inz(entrada: &[u8]) -> Result<(usize, Tokens), ()> {
        let analizador = seq(b"#INZ:");

        match analizador.parse(entrada) {
            Ok(saida) => {
                let mut resultado: Vec<u8> = Vec::new();

                for caractere in saida {
                    resultado.push(*caractere);
                }

                return Ok((resultado.len(), Tokens::AbreBlocoINZ));
            }

            Err(_e) => {
                return Err(());
            }
        };
    }

    pub fn fecha_bloco_inz(entrada: &[u8]) -> Result<(usize, Tokens), ()> {
        let analizador = seq(b"#INZ;");

        match analizador.parse(entrada) {
            Ok(saida) => {
                let mut resultado: Vec<u8> = Vec::new();

                for caractere in saida {
                    resultado.push(*caractere);
                }

                return Ok((resultado.len(), Tokens::FechaBlocoINZ));
            }

            Err(_e) => {
                return Err(());
            }
        };
    }

    pub fn abre_bloco_wnz(entrada: &[u8]) -> Result<(usize, Tokens), ()> {
        let analizador = seq(b"#WNZ:");

        match analizador.parse(entrada) {
            Ok(saida) => {
                let mut resultado: Vec<u8> = Vec::new();

                for caractere in saida {
                    resultado.push(*caractere);
                }

                return Ok((resultado.len(), Tokens::AbreBlocoWNZ));
            }

            Err(_e) => {
                return Err(());
            }
        };
    }

    pub fn fecha_bloco_wnz(entrada: &[u8]) -> Result<(usize, Tokens), ()> {
        let analizador = seq(b"#WNZ;");

        match analizador.parse(entrada) {
            Ok(saida) => {
                let mut resultado: Vec<u8> = Vec::new();

                for caractere in saida {
                    resultado.push(*caractere);
                }

                return Ok((resultado.len(), Tokens::FechaBlocoWNZ));
            }

            Err(_e) => {
                return Err(());
            }
        };
    }

    pub fn abre_bloco_rui(entrada: &[u8]) -> Result<(usize, Tokens), ()> {
        let analizador = seq(b"#RUI:");

        match analizador.parse(entrada) {
            Ok(saida) => {
                let mut resultado: Vec<u8> = Vec::new();

                for caractere in saida {
                    resultado.push(*caractere);
                }

                return Ok((resultado.len(), Tokens::AbreBlocoRUI));
            }

            Err(_e) => {
                return Err(());
            }
        };
    }

    pub fn fecha_bloco_rui(entrada: &[u8]) -> Result<(usize, Tokens), ()> {
        let analizador = seq(b"#RUI;");

        match analizador.parse(entrada) {
            Ok(saida) => {
                let mut resultado: Vec<u8> = Vec::new();

                for caractere in saida {
                    resultado.push(*caractere);
                }

                return Ok((resultado.len(), Tokens::FechaBlocoRUI));
            }

            Err(_e) => {
                return Err(());
            }
        };
    }

    pub fn operador(entrada: &[u8]) -> Result<(usize, Tokens), ()> {
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
                    Tokens::Operador(String::from_utf8(resultado).unwrap()),
                ));
            }

            Err(_e) => {
                return Err(());
            }
        };
    }

    pub fn abre_bloco_data(entrada: &[u8]) -> Result<(usize, Tokens), ()> {
        let analizador = seq(b"#DATA:");

        match analizador.parse(entrada) {
            Ok(saida) => {
                let mut resultado: Vec<u8> = Vec::new();

                for caractere in saida {
                    resultado.push(*caractere);
                }

                return Ok((resultado.len(), Tokens::AbreBlocoDATA));
            }

            Err(_e) => {
                return Err(());
            }
        };
    }

    pub fn fecha_bloco_data(entrada: &[u8]) -> Result<(usize, Tokens), ()> {
        let analizador = seq(b"#DATA;");

        match analizador.parse(entrada) {
            Ok(saida) => {
                let mut resultado: Vec<u8> = Vec::new();

                for caractere in saida {
                    resultado.push(*caractere);
                }

                return Ok((resultado.len(), Tokens::FechaBlocoDATA));
            }

            Err(_e) => {
                return Err(());
            }
        };
    }

    pub fn abre_bloco_main(entrada: &[u8]) -> Result<(usize, Tokens), ()> {
        let analizador = seq(b"#MAIN:");

        match analizador.parse(entrada) {
            Ok(saida) => {
                let mut resultado: Vec<u8> = Vec::new();

                for caractere in saida {
                    resultado.push(*caractere);
                }

                return Ok((resultado.len(), Tokens::AbreBlocoMAIN));
            }

            Err(_e) => {
                return Err(());
            }
        };
    }

    pub fn fecha_bloco_main(entrada: &[u8]) -> Result<(usize, Tokens), ()> {
        let analizador = seq(b"#MAIN;");

        match analizador.parse(entrada) {
            Ok(saida) => {
                let mut resultado: Vec<u8> = Vec::new();

                for caractere in saida {
                    resultado.push(*caractere);
                }

                return Ok((resultado.len(), Tokens::FechaBlocoMAIN));
            }

            Err(_e) => {
                return Err(());
            }
        };
    }

    pub fn abre_bloco_de_codigo(entrada: &[u8]) -> Result<(usize, Tokens), ()> {
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
                    Tokens::AbreBlocoDeCodigo(String::from_utf8(resultado).unwrap())
                ));
            }

            Err(_e) => {
                return Err(());
            }
        };
    }

    pub fn fecha_bloco_de_codigo(entrada: &[u8]) -> Result<(usize, Tokens), ()> {
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
                    Tokens::FechaBlocoDeCodigo(String::from_utf8(resultado).unwrap()),
                ));
            }

            Err(_e) => {
                return Err(());
            }
        };
    }

    pub fn tipo_de_variavel(entrada: &[u8]) -> Result<(usize, Tokens), ()> {
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
                    Tokens::TipoDeVariavel(String::from_utf8(resultado).unwrap()),
                ));
            }

            Err(_e) => {
                return Err(());
            }
        };
    }

    pub fn id_de_variavel(entrada: &[u8]) -> Result<(usize, Tokens), ()> {
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
                    Tokens::IdDeVariavel(String::from_utf8(resultado).unwrap()),
                ));
            }

            Err(_e) => {
                return Err(());
            }
        };
    }

    pub fn id_de_bloco(entrada: &[u8]) -> Result<(usize, Tokens), ()> {
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
                    Tokens::IdDeBloco(String::from_utf8(resultado).unwrap()),
                ));
            }

            Err(_e) => {
                return Err(());
            }
        };
    }

    pub fn virgula(entrada: &[u8]) -> Result<(usize, Tokens), ()> {
        let analizador = sym(b',');

        match analizador.parse(entrada) {
            Ok(_saida) => {
                return Ok((1, Tokens::Virgula));
            }

            Err(_e) => {
                return Err(());
            }
        };
    }

    pub fn ponto_e_virgula(entrada: &[u8]) -> Result<(usize, Tokens), ()> {
        let analizador = sym(b';');

        match analizador.parse(entrada) {
            Ok(_saida) => {
                return Ok((1, Tokens::PontoEVirgula));
            }

            Err(_e) => {
                return Err(());
            }
        };
    }

    pub fn abre_parenteses(entrada: &[u8]) -> Result<(usize, Tokens), ()> {
        let analizador = sym(b'(');

        match analizador.parse(entrada) {
            Ok(_saida) => {
                return Ok((1, Tokens::AbreParenteses));
            }

            Err(_e) => {
                return Err(());
            }
        };
    }

    pub fn fecha_parenteses(entrada: &[u8]) -> Result<(usize, Tokens), ()> {
        let analizador = sym(b')');

        match analizador.parse(entrada) {
            Ok(_saida) => {
                return Ok((1, Tokens::FechaParenteses));
            }

            Err(_e) => {
                return Err(());
            }
        };
    }

    pub fn dois_pontos(entrada: &[u8]) -> Result<(usize, Tokens), ()> {
        let analizador = sym(b':');

        match analizador.parse(entrada) {
            Ok(_saida) => {
                return Ok((1, Tokens::DoisPontos));
            }

            Err(_e) => {
                return Err(());
            }
        };
    }

    pub fn numero(entrada: &[u8]) -> Result<(usize, Tokens), ()> {
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
                    Tokens::Numero(String::from_utf8(resultado).unwrap()),
                ));
            }

            Err(_e) => {
                return Err(());
            }
        };
    }

    pub fn string(entrada: &[u8]) -> Result<(usize, Tokens), ()> {
        let mut resultado: Vec<u8> = Vec::new();

        if entrada[0] == b'"' {
            resultado.push(entrada[0].clone());
        } else {
            return Err(());
        }

        let mut j: usize = 1;
        while j < entrada.len() {
            if entrada[j] == b'"' {
                break;
            } else if entrada.len() > j + 1 && entrada[j] == b'\\' && entrada[j + 1] == b'\"' {
                resultado.push(entrada[j].clone());
                resultado.push(entrada[j + 1].clone());
            } else {
                resultado.push(entrada[j].clone());
            }
            j = resultado.len();
        }

        if entrada[j] != b'"' {
            return Err(());
        }

        resultado.remove(0); // remove " inicial

        let tamanho = resultado.len() + 2;
        let string = String::from_utf8(resultado).unwrap();
        let string_decodificada = decodificar_string(&string);

        return Ok((
            tamanho,
            Tokens::String(string_decodificada),
        ));
    }

    pub fn caractere(entrada: &[u8]) -> Result<(usize, Tokens), ()> {
        let entrada_inicio: Vec<u8>;

        if entrada.len() > 6 {
            entrada_inicio = entrada[0..6].to_vec();
        } else {
            entrada_inicio = entrada[0..(entrada.len() - 1)].to_vec();
        }

        let string = String::from_utf8(entrada_inicio).unwrap();
        let string_tamanho = string.len();

        let string_decodificada = decodificar_string(&string);
        let string_decodificada_tamanho = string_decodificada.len();

        let analizador = sym(b'\'') + any() + sym(b'\'');

        match analizador.parse(string_decodificada.as_bytes()) {
            Ok(saida) => {
                let mut resultado: Vec<u8> = Vec::new();

                resultado.push(saida.0.1);

                return Ok((
                    resultado.len() + 2 - (string_tamanho - string_decodificada_tamanho),
                    Tokens::Caractere(String::from_utf8(resultado).unwrap()),
                ));
            }

            Err(_e) => {
                return Err(());
            }
        };
    }

    fn decodificar_string(string: &str) -> String {
        let caracteres_especiais = vec![
            ("\\n", "\n"),
            ("\\t", "\t"),
            ("\\\\", "\\"),
            ("\\\"", "\""),
            ("\\\'", "\'"),
            ("\\r", "\r"),
            ("\\0", "\0"),
        ];
    
        let mut string_decodificada = string.to_string();
    
        for (representacao, real) in caracteres_especiais {
            string_decodificada = string_decodificada.replace(representacao, real);
        }
    
        return string_decodificada;
    }
    
}