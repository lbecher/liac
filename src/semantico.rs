use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use crate::{comum::*, llvm::*};

#[derive(Debug, Clone)]
pub struct Semantico {
    variaveis_globais: HashMap<String, TiposDeDado>,
    pilha_de_tokens: Vec<Tokens>,
    llvm: LLVM,
}

impl Semantico {
    pub fn inicializar() -> Self {
        Self {
            variaveis_globais: HashMap::new(),
            pilha_de_tokens: Vec::new(),
            llvm: LLVM::inicializar()
        }
    }

    pub fn gravar_llvm_ir(&self) {
        let resultado = self.llvm.obter_llvm_ir();

        // criando um arquivo e escrevendo o LLVM-IR nele
        if let Ok(mut file) = File::create("llvm.ll") {
            match file.write_all(resultado.as_bytes()) {
                Ok(_) => {}
                Err(_) => {
                    println!("Não foi possível gravar no arquivo de saída!");
                }
            }
        } else {
            println!("Não foi possível criar o arquivo de saída!");
        }
    }

    fn declarar_variaveis(&mut self) -> Result<(), String> {
        if let Some(Tokens::TipoDeVariavel(tipo_de_variavel)) = self.pilha_de_tokens.pop() {
            let tipo = match tipo_de_variavel.as_str() {
                "INT8" => {
                    TiposDeDado::Int8(None)
                }
                "UINT8" => {
                    TiposDeDado::Uint8(None)
                }
                "INT16" => {
                    TiposDeDado::Int16(None)
                }
                "UINT16" => {
                    TiposDeDado::Uint16(None)
                }
                "INT32" => {
                    TiposDeDado::Int32(None)
                }
                "UINT32" => {
                    TiposDeDado::Uint32(None)
                }
                "INT64" => {
                    TiposDeDado::Int64(None)
                }
                "UINT64" => {
                    TiposDeDado::Uint64(None)
                }
                "STR" => {
                    TiposDeDado::String(None)
                }
                "CHR" => {
                    TiposDeDado::Char(None)
                }
                _ => {
                    return Err(String::from("ERRO NO COMPILADOR: Inconsistência em Semantico::declarar_variaveis()!"));
                }
            };
            for tokens in self.pilha_de_tokens.iter() {
                if let Tokens::IdDeVariavel(id_de_variavel) = tokens {
                    if self.variaveis_globais.get(id_de_variavel.as_str()).is_some() {
                        return Err(format!("ERRO SEMÂNTICO: Variável '{}' já declarada!", id_de_variavel));
                    } else {
                        self.variaveis_globais.insert(id_de_variavel.to_string(), tipo.clone());
                        self.llvm.declarar_var_global(id_de_variavel.to_string(), tipo.clone());
                    }
                } else {
                    return Err(String::from("ERRO NO COMPILADOR: Inconsistência em Semantico::declarar_variaveis()!"));
                }
            }
            self.pilha_de_tokens.clear();
            return Ok(());
        } else {
            return Err(String::from("ERRO NO COMPILADOR: Inconsistência em Semantico::declarar_variaveis()!"));
        }
    }
    
    pub fn tratar_desempilhamento(&mut self, pilha: Vec<ElementosDaPilha>, estado: usize) -> Result<(), String> {
        for elemento in pilha {
            if let ElementosDaPilha::Tokens(tokens) = elemento {
                match tokens {
                    Tokens::TipoDeVariavel(tipo_de_variavel) => {
                        self.pilha_de_tokens.push(Tokens::TipoDeVariavel(tipo_de_variavel));
                    }
                    Tokens::IdDeVariavel(ide_de_variavel) => {
                        self.pilha_de_tokens.push(Tokens::IdDeVariavel(ide_de_variavel));
                    }
                    _ => {}
                }
            }
        }

        match estado {
            10 => {
                return self.declarar_variaveis();
            }
            _ => {}
        }
        Ok(())
    }
}