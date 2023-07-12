use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use liac::*;
use crate::llvm::*;

#[derive(Debug, Clone)]
pub struct Semantico {
    variaveis_globais: HashMap<String, ParametroLLVM>,
    blocos_de_codigos: HashMap<String, bool>,
    pilha_de_tokens: Vec<Tokens>,
    pilha_de_parametros: Vec<ParametroLLVM>,
    retorno_de_operador: Option<ParametroLLVM>,
    llvm: LLVM,
}

impl Semantico {
    pub fn inicializar() -> Self {
        Self {
            variaveis_globais: HashMap::new(),
            blocos_de_codigos: HashMap::new(),
            pilha_de_tokens: Vec::new(),
            pilha_de_parametros: Vec::new(),
            retorno_de_operador: None,
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
                    Tokens::IdDeVariavel(id_de_variavel) => {
                        self.pilha_de_tokens.push(Tokens::IdDeVariavel(id_de_variavel));
                    }
                    Tokens::IdDeBloco(id_de_bloco) => {
                        self.pilha_de_tokens.push(Tokens::IdDeBloco(id_de_bloco));
                    }
                    Tokens::AbreBlocoDeCodigo(id_de_bloco) => {
                        self.pilha_de_tokens.push(Tokens::AbreBlocoDeCodigo(id_de_bloco));
                    }
                    Tokens::Operador(operador) => {
                        self.pilha_de_tokens.push(Tokens::Operador(operador));
                    }
                    Tokens::Numero(numero) => {
                        self.pilha_de_tokens.push(Tokens::Numero(numero));
                    }
                    Tokens::String(string) => {
                        self.pilha_de_tokens.push(Tokens::String(string));
                    }
                    Tokens::Caractere(caractere) => {
                        self.pilha_de_tokens.push(Tokens::Caractere(caractere));
                    }
                    _ => {}
                }
            }
        }

        match estado {
            4 => {
                self.llvm.gerar_bloco_main();
                return Ok(());
            }
            7 => {
                println!(":::\n{:?}\n{:?}", self.pilha_de_tokens, self.pilha_de_parametros);
                if let Some(Tokens::AbreBlocoDeCodigo(bloco)) = self.pilha_de_tokens.pop() {
                    if let Some(declarado) = self.blocos_de_codigos.get_mut(bloco.as_str()) {
                        *declarado = true;
                    } else {
                        self.blocos_de_codigos.insert(bloco.to_string(), true);
                    }

                    self.llvm.gerar_bloco(bloco.as_str());

                    return Ok(());
                } else {
                    return Err(String::from("ERRO NO COMPILADOR: Sei lá!"));
                }
            }
            10 => {
                return self.declarar_variaveis();
            }
            18 => {
                return self.gerar_chamada_de_bloco();
            }
            19 => {
                return self.modificar_variavel();
            }
            20 => {
                return self.gerar_print();
            }
            21 => {
                return self.gerar_print();
            }
            22 => {
                return self.gerar_scan();
            }
            28 => {
                return self.empilhar_parametros_do_print();
            }
            29 => {
                return self.empilhar_parametros_do_print();
            }
            30 => {
                return self.empilhar_parametros_do_print();
            }
            31 => {
                return self.empilhar_parametros_do_print();
            }
            32 => {
                return self.empilhar_parametros_do_print();
            }
            33 => {
                return self.empilhar_parametros_do_print();
            }
            34 => {
                return self.empilhar_parametros_do_print();
            }
            35 => {
                return self.empilhar_parametros_do_print();
            }
            36 => {
                return self.empilhar_parametros_do_print();
            }
            37 => {
                return self.empilhar_parametros_do_print();
            }
            38 => {
                return self.empilhar_parametros_do_scan();
            }
            39 => {
                return self.empilhar_parametros_do_scan();
            }
            /*=> {
                //return self.operacao_matematica();
            }*/
            _ => {}
        }
        Ok(())
    }

    pub fn gravar_codigo_llvm(&self) {
        let resultado = self.llvm.obter_codigo_llvm();

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
                    TipoDeDado::Int8
                }
                "UINT8" => {
                    TipoDeDado::Uint8
                }
                "INT16" => {
                    TipoDeDado::Int16
                }
                "UINT16" => {
                    TipoDeDado::Uint16
                }
                "INT32" => {
                    TipoDeDado::Int32
                }
                "UINT32" => {
                    TipoDeDado::Uint32
                }
                "INT64" => {
                    TipoDeDado::Int64
                }
                "UINT64" => {
                    TipoDeDado::Uint64
                }
                "STR" => {
                    TipoDeDado::String
                }
                "CHR" => {
                    TipoDeDado::Char
                }
                _ => {
                    return Err(String::from("ERRO NO COMPILADOR: Inconsistência em Semantico::declarar_variaveis()!"));
                }
            };

            for tokens in self.pilha_de_tokens.iter() {
                if let Tokens::IdDeVariavel(id_de_variavel) = tokens {
                    if self.variaveis_globais.get(id_de_variavel.as_str()).is_some() {
                        return Err(format!("ERRO SEMÂNTICO: Variável @{} já declarada!", id_de_variavel));
                    } else {
                        self.variaveis_globais.insert(
                            id_de_variavel.to_string(),
                            ParametroLLVM::instanciar(
                                id_de_variavel.as_str(),
                                TipoDeParametroLLVM::VariavelGlobal,
                                tipo.clone(),
                            ),
                        );

                        self.llvm.declarar_variavel_global(id_de_variavel, tipo.clone());
                    }
                } else {
                    return Err(String::from("ERRO NO COMPILADOR: Inconsistência em Semantico::declarar_variaveis()!"));
                }
            }

            self.pilha_de_tokens.clear();
        } else {
            return Err(String::from("ERRO NO COMPILADOR: Inconsistência em Semantico::declarar_variaveis()!"));
        }

        return Ok(());
    }

    fn modificar_variavel(&mut self) -> Result<(), String> {
        let id_var_de_destino: String = match self.pilha_de_tokens.pop() {
            Some(Tokens::IdDeVariavel(id)) => {
                id.to_string()
            }
            _ => {
                return Err(String::from(
                    "ERRO NO COMPILADOR: Inconsistência em Semantico::modificar_variavel()!"
                ));
            }
        };

        let var_de_destino: ParametroLLVM = match self.variaveis_globais.get(id_var_de_destino.as_str()) {
            Some(var) => {
                var.clone()
            },
            None => {
                return Err(format!(
                    "ERRO SEMÂNTICO: Variável @{} não declarada!",
                    id_var_de_destino,
                ));
            }
        };

        let proximo_token = self.pilha_de_tokens.pop();

        let atribuicao: ParametroLLVM = match proximo_token.clone() {
            Some(Tokens::Numero(numero)) => {
                match Semantico::numero_compativel(
                    TipoDeDado::Int32,
                    numero.as_str(),
                ) {
                    Ok(compativel) => {
                        if !compativel {
                            return Err(format!("ERRO SEMÂNTICO: O número '{}' não é compatível com {:?}!", numero, TipoDeDado::Int32));
                        }
                    }
                    Err(erro) => return Err(erro),
                }

                ParametroLLVM::instanciar(
                    numero.as_str(),
                    TipoDeParametroLLVM::Numero,
                    var_de_destino.tipo_de_dado.clone(),
                )
            }
            Some(Tokens::Caractere(caracter)) => {
                if var_de_destino.tipo_de_dado.clone() != TipoDeDado::Char {
                    return Err(format!(
                        "ERRO SEMÂNTICO: O caracter '{}' não é compatível com {:?}!", 
                        caracter, 
                        var_de_destino.tipo_de_dado.clone(),
                    ));
                }

                ParametroLLVM::instanciar(
                    caracter.as_str(),
                    TipoDeParametroLLVM::Caractere,
                    TipoDeDado::Char,
                )
            }
            Some(Tokens::String(string)) => {
                if var_de_destino.tipo_de_dado.clone() != TipoDeDado::Char {
                    return Err(format!(
                        "ERRO SEMÂNTICO: A string '{}' não é compatível com {:?}!", 
                        string, 
                        var_de_destino.tipo_de_dado.clone(),
                    ));
                }

                ParametroLLVM::instanciar(
                    string.as_str(),
                    TipoDeParametroLLVM::String,
                    TipoDeDado::String,
                )
            }
            Some(Tokens::IdDeVariavel(id)) => {
                if let Some(var_global) = self.variaveis_globais.get(id.as_str()) {
                    if var_de_destino.tipo_de_dado.clone() != var_global.tipo_de_dado.clone() {
                        return Err(format!(
                            "ERRO SEMÂNTICO: a variável {} não é compatível com {:?}!",
                            var_de_destino.parametro,
                            var_global.tipo_de_dado,
                        ));
                    }

                    var_global.clone()
                } else {
                    return Err(format!(
                        "ERRO SEMÂNTICO: Variável @{} não declarada!",
                        id,
                    ));
                }
            }
            _ => {
                if let Some(pt) = proximo_token {
                    self.pilha_de_tokens.push(pt);
                };

                if let Some(retorno_de_operador) = self.retorno_de_operador.clone() {
                    if var_de_destino.tipo_de_dado.clone() != retorno_de_operador.tipo_de_dado.clone() {
                        return Err(format!(
                            "ERRO SEMÂNTICO: a variável {} não é compatível com {:?}!",
                            var_de_destino.parametro,
                            retorno_de_operador.tipo_de_dado,
                        ));
                    }

                    self.retorno_de_operador = None;
                    retorno_de_operador
                } else {
                    return Err(String::from("ERRO NO COMPILADOR: Inconsistência em Semantico::modificar_variavel()!"));
                }
            }
        };

        self.llvm.gerar_atribuicao(var_de_destino, atribuicao);

        Ok(())
    }

    fn gerar_print(&mut self) -> Result<(), String> {
        if let Some(Tokens::String(mascara_string)) = self.pilha_de_tokens.pop() {
            let mascara = self.llvm.declarar_string(
                ParametroLLVM::instanciar(
                    mascara_string.as_str(),
                    TipoDeParametroLLVM::String,
                    TipoDeDado::String,
                )
            );

            let mut parametros = self.pilha_de_parametros.to_vec();
            parametros.reverse();
            self.pilha_de_parametros.clear();

            self.llvm.gerar_print(mascara, parametros);
        } else {
            return Err(String::from("ERRO NO COMPILADOR: Inconsistência em Semantico::gerar_print()!"));
        }

        Ok(())
    }

    fn gerar_scan(&mut self) -> Result<(), String> {
        if let Some(Tokens::String(mascara_string)) = self.pilha_de_tokens.pop() {
            let mascara = self.llvm.declarar_string(
                ParametroLLVM::instanciar(
                    mascara_string.as_str(),
                    TipoDeParametroLLVM::String,
                    TipoDeDado::String,
                )
            );

            let mut parametros = self.pilha_de_parametros.to_vec();
            parametros.reverse();
            self.pilha_de_parametros.clear();

            self.llvm.gerar_scan(mascara, parametros);
        } else {
            return Err(String::from("ERRO NO COMPILADOR: Inconsistência em Semantico::gerar_scan()!"));
        }

        Ok(())
    }

    fn gerar_chamada_de_bloco(&mut self) -> Result<(), String> {
        if let Some(Tokens::IdDeBloco(bloco)) = self.pilha_de_tokens.pop() {
            if self.blocos_de_codigos.get(bloco.as_str()).is_none() {
                self.blocos_de_codigos.insert(bloco.to_string(), true);
            }

            self.llvm.gerar_chamada_de_bloco(ParametroLLVM::instanciar(
                bloco.as_str(),
                TipoDeParametroLLVM::Bloco,
                TipoDeDado::Undefined,
            ));
            
            return Ok(());
        } else {
            return Err(String::from("ERRO NO COMPILADOR: Sei lá!"));
        }
    }

    fn empilhar_parametros_do_scan(&mut self) -> Result<(), String> {
        let token = self.pilha_de_tokens.pop();

        let parametro: ParametroLLVM = match token.clone() {
            Some(Tokens::IdDeVariavel(id)) => {
                if let Some(var_global) = self.variaveis_globais.get(id.as_str()) {
                    var_global.clone()
                } else {
                    return Err(format!(
                        "ERRO SEMÂNTICO: Variável @{} não declarada!",
                        id,
                    ));
                }
            }
            _ => {
                return Err(String::from("ERRO NO COMPILADOR: Inconsistência em Semantico::empilhar_parametros_do_scan()!"));
            }
        };

        self.pilha_de_parametros.push(parametro);

        Ok(())
    }

    fn empilhar_parametros_do_print(&mut self) -> Result<(), String> {
        let token = self.pilha_de_tokens.pop();

        let parametro: ParametroLLVM = match token.clone() {
            Some(Tokens::Numero(numero)) => {
                match Semantico::numero_compativel(
                    TipoDeDado::Int32,
                    numero.as_str(),
                ) {
                    Ok(compativel) => {
                        if !compativel {
                            return Err(format!("ERRO SEMÂNTICO: O número '{}' não é compatível com {:?}!", numero, TipoDeDado::Int32));
                        }
                    }
                    Err(erro) => return Err(erro),
                }

                ParametroLLVM::instanciar(
                    numero.as_str(),
                    TipoDeParametroLLVM::Numero,
                    TipoDeDado::Int32,
                )
            }
            Some(Tokens::Caractere(caracter)) => {
                ParametroLLVM::instanciar(
                    caracter.as_str(),
                    TipoDeParametroLLVM::Caractere,
                    TipoDeDado::Char,
                )
            }
            Some(Tokens::String(string)) => {
                ParametroLLVM::instanciar(
                    string.as_str(),
                    TipoDeParametroLLVM::String,
                    TipoDeDado::String,
                )
            }
            Some(Tokens::IdDeVariavel(id)) => {
                if let Some(var_global) = self.variaveis_globais.get(id.as_str()) {
                    var_global.clone()
                } else {
                    return Err(format!(
                        "ERRO SEMÂNTICO: Variável @{} não declarada!",
                        id,
                    ));
                }
            }
            _ => {
                if let Some(t) = token {
                    self.pilha_de_tokens.push(t);
                };

                if let Some(retorno_de_operador) = self.retorno_de_operador.clone() {
                    self.retorno_de_operador = None;
                    retorno_de_operador
                } else {
                    return Err(String::from("ERRO NO COMPILADOR: Inconsistência em Semantico::empilhar_parametros_do_print()!"));
                }
            }
        };

        self.pilha_de_parametros.push(parametro);

        Ok(())
    }
/*
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
                let operador2_heranca: Option<ParametroLLVM> = self.pilha_de_heranca.pop();

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
                                if operador1.tipo.clone() == operador2.tipo_de_dado.clone() {
                                    let retorno = self.llvm.gerar_operacao(
                                        ParametroLLVM::instanciar(
                                            id_de_variavel.as_str(),
                                            TipoDeParametroLLVM::VariavelGlobal,
                                            operador1.tipo.clone(),
                                        ),
                                        operador,
                                        Some(operador2),
                                    );
                                    self.pilha_de_heranca.push(retorno);
                                } else {
                                    return Err(String::from("ERRO SEMÂNTICO: Parâmetros de tipos diferentes!"));
                                }
                            } else {
                                if let Some(Tokens::IdDeVariavel(id_de_var_operador2)) = operador2_token {
                                    if let Some(operador2) = self.variaveis_globais.get(id_de_var_operador2.as_str()) {
                                        if operador1.tipo.clone() == operador2.tipo.clone() {
                                            let retorno = self.llvm.gerar_operacao(
                                                ParametroLLVM::instanciar(
                                                    id_de_variavel.as_str(),
                                                    TipoDeParametroLLVM::VariavelGlobal,
                                                    operador1.tipo.clone(),
                                                ),
                                                operador,
                                                Some(ParametroLLVM::instanciar(
                                                    id_de_var_operador2.as_str(),
                                                    TipoDeParametroLLVM::VariavelGlobal,
                                                    operador2.tipo.clone(),
                                                )),
                                            );
                                            self.pilha_de_heranca.push(retorno);
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
                                                let retorno = self.llvm.gerar_operacao(
                                                    ParametroLLVM::instanciar(
                                                        id_de_variavel.as_str(),
                                                        TipoDeParametroLLVM::VariavelGlobal,
                                                        operador1.tipo.clone(),
                                                    ),
                                                    operador,
                                                    Some(ParametroLLVM::instanciar(
                                                        numero.as_str(),
                                                        TipoDeParametroLLVM::VariavelGlobal,
                                                        operador1.tipo.clone(),
                                                    )),
                                                );
                                                self.pilha_de_heranca.push(retorno);
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
*/
    fn numero_compativel(tipo: TipoDeDado, numero: &str) -> Result<bool, String> {
        let compativel = match tipo {
            TipoDeDado::Int8 => {
                let resultado: Result<i8, _> = numero.to_string().parse();
                match resultado {
                    Ok(_) => true,
                    Err(_) => false,
                }
            }
            TipoDeDado::Uint8 => {
                let resultado: Result<u8, _> = numero.to_string().parse();
                match resultado {
                    Ok(_) => true,
                    Err(_) => false,
                }
            }
            TipoDeDado::Int16 => {
                let resultado: Result<i16, _> = numero.to_string().parse();
                match resultado {
                    Ok(_) => true,
                    Err(_) => false,
                }
            }
            TipoDeDado::Uint16 => {
                let resultado: Result<u16, _> = numero.to_string().parse();
                match resultado {
                    Ok(_) => true,
                    Err(_) => false,
                }
            }
            TipoDeDado::Int32 => {
                let resultado: Result<i32, _> = numero.to_string().parse();
                match resultado {
                    Ok(_) => true,
                    Err(_) => false,
                }
            }
            TipoDeDado::Uint32 => {
                let resultado: Result<u32, _> = numero.to_string().parse();
                match resultado {
                    Ok(_) => true,
                    Err(_) => false,
                }
            }
            TipoDeDado::Int64 => {
                let resultado: Result<i64, _> = numero.to_string().parse();
                match resultado {
                    Ok(_) => true,
                    Err(_) => false,
                }
            }
            TipoDeDado::Uint64 => {
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