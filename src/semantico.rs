use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use crate::{comum::*, llvm::*};

#[derive(Debug, Clone)]
pub struct Semantico {
    variaveis_globais: HashMap<String, VariavelGlobal>,
    pilha_de_tokens: Vec<Tokens>,
    pilha_de_heranca: Vec<VariavelTemporaria>,
    llvm: LLVM,
}

impl Semantico {
    pub fn inicializar() -> Self {
        Self {
            variaveis_globais: HashMap::new(),
            pilha_de_tokens: Vec::new(),
            pilha_de_heranca: Vec::new(),
            llvm: LLVM::inicializar(),
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
                    Tokens::Operador(operador) => {
                        self.pilha_de_tokens.push(Tokens::Operador(operador));
                    }
                    Tokens::Numero(numero) => {
                        self.pilha_de_tokens.push(Tokens::Numero(numero));
                    }
                    _ => {}
                }
            }
        }

        match estado {
            4 => {
                self.llvm.criar_funcao_main();
                return Ok(());
            }
            10 => {
                return self.declarar_variaveis();
            }
            19 => {
                return self.modificar_variavel();
            }
            34 => {
                return self.operacao_matematica();
            }
            _ => {}
        }
        Ok(())
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
                    TiposDeDado::Int8
                }
                "UINT8" => {
                    TiposDeDado::Uint8
                }
                "INT16" => {
                    TiposDeDado::Int16
                }
                "UINT16" => {
                    TiposDeDado::Uint16
                }
                "INT32" => {
                    TiposDeDado::Int32
                }
                "UINT32" => {
                    TiposDeDado::Uint32
                }
                "INT64" => {
                    TiposDeDado::Int64
                }
                "UINT64" => {
                    TiposDeDado::Uint64
                }
                "STR" => {
                    TiposDeDado::String
                }
                "CHR" => {
                    TiposDeDado::Char
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
                        self.variaveis_globais.insert(
                            id_de_variavel.to_string(),
                            VariavelGlobal {
                                tipo: tipo.clone(),
                                inicializada: false,
                            }
                        );
                        self.llvm.declarar_var_global(id_de_variavel, tipo.clone());
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

    fn modificar_variavel(&mut self) -> Result<(), String> {
        if let Some(Tokens::IdDeVariavel(id_de_variavel)) = self.pilha_de_tokens.pop() {
            if let Some(variavel_global) = self.variaveis_globais.get(id_de_variavel.as_str()) {
                if let Some(token) = self.pilha_de_tokens.pop() {
                    match token {
                        Tokens::Numero(numero) => {
                            match Self::numero_compativel(variavel_global.tipo.clone(), numero.as_str()) {
                                Ok(compativel) => {
                                    if compativel {
                                        self.llvm.modificar_variavel(
                                            id_de_variavel.as_str(),
                                            variavel_global.tipo.clone(),
                                            ParametroGenerico::Imediato(numero),
                                        );
                                        self.marca_var_como_inicializada(id_de_variavel.as_str());
                                    } else {
                                        return Err(format!("ERRO SEMÂNTICO: O número '{}' não é compatível com {:?}!", numero, variavel_global.tipo.clone()));
                                    }
                                }
                                Err(erro) => {
                                    return Err(erro);
                                }
                            }
                        }
                        Tokens::IdDeVariavel(id_de_variavel_do_parametro) => {
                            if let Some(parametro) = self.variaveis_globais.get(id_de_variavel_do_parametro.as_str()) {
                                if !parametro.inicializada {
                                    return Err(format!("ERRO SEMÂNTICO: Variável '{}' não inicializada!", id_de_variavel_do_parametro));
                                }
                                if variavel_global.tipo.clone() == parametro.tipo.clone() {
                                    self.llvm.modificar_variavel(
                                        id_de_variavel.as_str(),
                                        variavel_global.tipo.clone(),
                                        ParametroGenerico::VariavelGlobal(id_de_variavel_do_parametro),
                                    );
                                    self.marca_var_como_inicializada(id_de_variavel.as_str());
                                } else {
                                    return Err(String::from("ERRO SEMÂNTICO: Parâmetros de tipos diferentes!"));
                                }
                            } else {
                                return Err(format!("ERRO SEMÂNTICO: Variável '{}' não declarada!", id_de_variavel_do_parametro));
                            }
                        }
                        _ => {
                            return Err(String::from("ERRO NO COMPILADOR: Inconsistência em Semantico::modificar_variavel()!"));
                        }
                    }
                } else if let Some(var_temporaria) = self.pilha_de_heranca.pop() {
                    if variavel_global.tipo.clone() == var_temporaria.tipo.clone() {
                        self.llvm.modificar_variavel(
                            id_de_variavel.as_str(),
                            variavel_global.tipo.clone(),
                            ParametroGenerico::VariavelTemporaria(var_temporaria),
                        );
                    } else {
                        return Err(format!("ERRO SEMÂNTICO: Variável '{}' não possui o mesmo tipo de dado da atribuição!", id_de_variavel));
                    }
                } else {
                    return Err(String::from("ERRO NO COMPILADOR: Inconsistência em Semantico::modificar_variavel()!"));
                }
            } else {
                return Err(format!("ERRO SEMÂNTICO: Variável '{}' não declarada!", id_de_variavel));
            }
        } else {
            return Err(String::from("ERRO NO COMPILADOR: Inconsistência em Semantico::modificar_variavel()!"));
        }
        Ok(())
    }

    fn marca_var_como_inicializada(&mut self, var: &str) {
        if let Some(mut variavel_global) = self.variaveis_globais.get_mut(var) {
            variavel_global.inicializada = true;
        }
    }

    fn operacao_matematica(&mut self) -> Result<(), String> {
        if let Some(Tokens::Operador(operador)) = self.pilha_de_tokens.pop() {
            let operador = match operador.as_str() {
                "ADD" => {
                    Operador::ADD
                }
                "SUB" => {
                    Operador::SUB
                }
                "MUL" => {
                    Operador::MUL
                }
                "DIV" => {
                    Operador::DIV
                }
                "DIVR" => {
                    Operador::DIVR
                }
                "OR" => {
                    Operador::OR
                }
                "AND" => {
                    Operador::AND
                }
                "NOT" => {
                    Operador::NOT
                }
                _ => {
                    return Err(String::from("ERRO NO COMPILADOR: Inconsistência em Semantico::operacao_matematica():5!"));
                }
            };
            if operador == Operador::NOT {

            } else {
                println!("{:?}", self.pilha_de_tokens);
                println!("{:?}", self.pilha_de_heranca);

                let mut operador2_token: Option<Tokens> = None;
                let operador2_heranca = self.pilha_de_heranca.pop();

                if operador2_heranca.is_none() {
                    operador2_token = self.pilha_de_tokens.pop();
                }

                let operador1 = self.pilha_de_tokens.pop();

                match operador1.unwrap() {
                    Tokens::IdDeVariavel(id_de_variavel) => {
                        if let Some(operador1) = self.variaveis_globais.get(id_de_variavel.as_str()) {
                            if !operador1.inicializada {
                                return Err(format!("ERRO SEMÂNTICO: Variável '{}' não inicializada!", id_de_variavel));
                            }

                            if let Some(operador2) = operador2_heranca {
                                if operador1.tipo.clone() == operador2.tipo.clone() {
                                    let retorno = self.llvm.adicionar_operacao(
                                        id_de_variavel.as_str(),
                                        operador,
                                        Some(ParametroGenerico::VariavelTemporaria(operador2)),
                                        operador1.tipo.clone(),
                                    );
                                    self.pilha_de_heranca.push(VariavelTemporaria {
                                        nome: retorno.0,
                                        tipo: retorno.1,
                                    })
                                } else {
                                    return Err(String::from("ERRO SEMÂNTICO: Parâmetros de tipos diferentes!"));
                                }
                            } else {
                                if let Some(Tokens::IdDeVariavel(id_de_var_operador2)) = operador2_token {
                                    if let Some(operador2) = self.variaveis_globais.get(id_de_var_operador2.as_str()) {
                                        if operador1.tipo.clone() == operador2.tipo.clone() {
                                            let retorno = self.llvm.adicionar_operacao(
                                                id_de_variavel.as_str(),
                                                operador,
                                                Some(ParametroGenerico::VariavelGlobal(id_de_var_operador2)),
                                                operador1.tipo.clone(),
                                            );
                                            self.pilha_de_heranca.push(VariavelTemporaria {
                                                nome: retorno.0,
                                                tipo: retorno.1,
                                            })
                                        } else {
                                            return Err(String::from("ERRO SEMÂNTICO: Parâmetros de tipos diferentes!"));
                                        }
                                    } else {
                                        return Err(String::from("ERRO NO COMPILADOR: Inconsistência em Semantico::operacao_matematica():4!"));
                                    }
                                } else if let Some(Tokens::Numero(numero)) = operador2_token {
                                    match Self::numero_compativel(operador1.tipo.clone(), numero.as_str()) {
                                        Ok(compativel) => {
                                            if compativel {
                                                let retorno = self.llvm.adicionar_operacao(
                                                    id_de_variavel.as_str(),
                                                    operador,
                                                    Some(ParametroGenerico::Imediato(numero)),
                                                    operador1.tipo.clone(),
                                                );
                                                self.pilha_de_heranca.push(VariavelTemporaria {
                                                    nome: retorno.0,
                                                    tipo: retorno.1,
                                                })
                                            } else {
                                                return Err(format!("ERRO SEMÂNTICO: O número '{}' não é compatível com {:?}!", numero, operador1.tipo.clone()));
                                            }
                                        }
                                        Err(erro) => {
                                            return Err(erro);
                                        }
                                    }
                                } else {
                                    return Err(String::from("ERRO NO COMPILADOR: Inconsistência em Semantico::operacao_matematica():3!"));
                                }
                            }
                        } else {
                            return Err(format!("ERRO SEMÂNTICO: Variável '{}' não declarada!", id_de_variavel));
                        }
                    }
                    _ => {
                        return Err(String::from("ERRO NO COMPILADOR: Inconsistência em Semantico::operacao_matematica():2!"));
                    }
                }
            }
            return Ok(());
        } else {
            return Err(String::from("ERRO NO COMPILADOR: Inconsistência em Semantico::operacao_matematica():1!"));
        }
    }
    
    fn numero_compativel(tipo: TiposDeDado, numero: &str) -> Result<bool, String> {
        let compativel = match tipo {
            TiposDeDado::Int8 => {
                let resultado: Result<i8, _> = numero.to_string().parse();
                match resultado {
                    Ok(_) => true,
                    Err(_) => false,
                }
            }
            TiposDeDado::Uint8 => {
                let resultado: Result<u8, _> = numero.to_string().parse();
                match resultado {
                    Ok(_) => true,
                    Err(_) => false,
                }
            }
            TiposDeDado::Int16 => {
                let resultado: Result<i16, _> = numero.to_string().parse();
                match resultado {
                    Ok(_) => true,
                    Err(_) => false,
                }
            }
            TiposDeDado::Uint16 => {
                let resultado: Result<u16, _> = numero.to_string().parse();
                match resultado {
                    Ok(_) => true,
                    Err(_) => false,
                }
            }
            TiposDeDado::Int32 => {
                let resultado: Result<i32, _> = numero.to_string().parse();
                match resultado {
                    Ok(_) => true,
                    Err(_) => false,
                }
            }
            TiposDeDado::Uint32 => {
                let resultado: Result<u32, _> = numero.to_string().parse();
                match resultado {
                    Ok(_) => true,
                    Err(_) => false,
                }
            }
            TiposDeDado::Int64 => {
                let resultado: Result<i64, _> = numero.to_string().parse();
                match resultado {
                    Ok(_) => true,
                    Err(_) => false,
                }
            }
            TiposDeDado::Uint64 => {
                let resultado: Result<u64, _> = numero.to_string().parse();
                match resultado {
                    Ok(_) => true,
                    Err(_) => false,
                }
            }
            _ => {
                return Err(String::from("ERRO NO COMPILADOR: Inconsistência em Semantico::numero_compativel()!"));
            }
        };
        Ok(compativel)
    }
}