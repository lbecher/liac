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