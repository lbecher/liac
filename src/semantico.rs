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
    pilha_de_parametros_condicionais: Vec<ParametroLLVM>,
    retorno_de_operador: Option<ParametroLLVM>,
    llvm: LLVM,
    pilha_de_blocos: Vec<Tokens>,
}

impl Semantico {
    pub fn inicializar() -> Self {
        Self {
            variaveis_globais: HashMap::new(),
            blocos_de_codigos: HashMap::new(),
            pilha_de_tokens: Vec::new(),
            pilha_de_parametros: Vec::new(),
            pilha_de_parametros_condicionais: Vec::new(),
            retorno_de_operador: None,
            llvm: LLVM::inicializar(),
            pilha_de_blocos: Vec::new(),
        }
    }

    pub fn empilha_bloco(&mut self, bloco_de_codigo: Tokens) {
        self.pilha_de_blocos.push(bloco_de_codigo);
        self.llvm.bloco_basico_para_bloco_de_funcao();
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

        //println!("{}", estado);
        //println!("{:?}", self.pilha_de_tokens);
        //println!("{:?}", self.pilha_de_parametros);
        //println!("{:?}", self.retorno_de_operador);

        match estado {
            1 => {
                //println!("{:?}", self.blocos_de_codigos);
                return self.verificar_declaracao_de_blocos();
            }
            2 => {
                //println!("{:?}", self.blocos_de_codigos);
                return self.verificar_declaracao_de_blocos();
            }
            4 => {
                self.llvm.bloco_basico_para_bloco_de_funcao();
                self.llvm.gerar_bloco_main();
                return Ok(());
            }
            7 => {
                if let Some(Tokens::AbreBlocoDeCodigo(bloco)) = self.pilha_de_tokens.pop() {
                    if let Some(declarado) = self.blocos_de_codigos.get_mut(bloco.as_str()) {
                        *declarado = true;
                    } else {
                        self.blocos_de_codigos.insert(bloco.to_string(), true);
                    }

                    self.llvm.bloco_basico_para_bloco_de_funcao();
                    self.llvm.gerar_bloco(bloco.as_str());

                    return Ok(());
                } else {
                    return Err(String::from("ERRO NO COMPILADOR: Sei lá!"));
                }
            }
            10 => {
                return self.declarar_variaveis();
            }
            15 => {
                self.pilha_de_blocos.pop();
                return self.gerar_inz();
            }
            16 => {
                self.pilha_de_blocos.pop();
                return self.gerar_wnz();
            }
            18 => {
                return self.gerar_chamada_de_bloco();
            }
            19 => {
                return self.modificar_variavel();
            }

            // PRINT
            20 => {
                return self.gerar_print();
            }
            21 => {
                return self.gerar_print();
            }

            // SCAN
            22 => {
                return self.gerar_scan();
            }

            // RETURN
            23 => {
                self.llvm.gerar_retorno();
                return Ok(());
            }

            // parametros do SET
            24 => {
                return self.empilhar_parametro_do_set();
            }
            25 => {
                return self.empilhar_parametro_do_set();
            }
            26 => {
                return self.empilhar_parametro_do_set();
            }
            27 => {
                return self.empilhar_parametro_do_set();
            }
            28 => {
                return self.empilhar_parametro_do_set();
            }

            // parametros do PRINT
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
                return self.empilhar_parametros_do_print();
            }

            // parametros do SCAN
            39 => {
                return self.empilhar_parametros_do_scan();
            }
            40 => {
                return self.empilhar_parametros_do_scan();
            }
            
            // operação
            41 => {
                return self.gerar_operacao();
            }
            42 => {
                return self.gerar_operacao();
            }

            // segundo parametro da operação
            43 => {
                return self.obter_operando2_de_operacao();
            }
            44 => {
                return self.obter_operando2_de_operacao();
            }
            45 => {
                return self.obter_operando2_de_operacao();
            }

            // parametro de comparação do INZ/WNZ
            46 => {
                return self.empilhar_parametro_de_comparacao();
            }
            47 => {
                return self.empilhar_parametro_de_comparacao();
            }

            _ => {}
        }
        Ok(())
    }

    fn verificar_declaracao_de_blocos(&mut self) -> Result<(), String> {
        for (bloco, declarado) in self.blocos_de_codigos.clone() {
            if !declarado {
                return Err(format!("ERRO SEMÂNTICO: Bloco '{}' não declarado!", bloco));
            }
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

        let atribuicao: ParametroLLVM = match self.pilha_de_parametros.pop() {
            Some(mut parametro) => {
                if parametro.tipo_de_parametro.clone() == TipoDeParametroLLVM::Numero {
                    match Semantico::numero_compativel(
                        var_de_destino.tipo_de_dado.clone(),
                        parametro.parametro.as_str(),
                    ) {
                        Ok(compativel) => {
                            if !compativel {
                                return Err(format!(
                                    "ERRO SEMÂNTICO: O número '{}' não é compatível com {:?}!",
                                    parametro.parametro,
                                    var_de_destino.tipo_de_dado.clone(),
                                ));
                            }

                            parametro = ParametroLLVM::instanciar(
                                parametro.parametro.as_str(),
                                TipoDeParametroLLVM::Numero,
                                var_de_destino.tipo_de_dado.clone(),
                            )
                        }
                        Err(erro) => return Err(erro),
                    }
                } else if parametro.tipo_de_parametro.clone() == TipoDeParametroLLVM::VariavelGlobal {
                    if parametro.tipo_de_dado.clone() != var_de_destino.tipo_de_dado.clone() {
                        return Err(format!(
                            "ERRO SEMÂNTICO: A variável {} não é compatível com {:?}!",
                            parametro.parametro,
                            var_de_destino.tipo_de_dado.clone(),
                        ));
                    }
                } else if parametro.tipo_de_parametro.clone() == TipoDeParametroLLVM::VariavelTemporaria {
                    if parametro.tipo_de_dado.clone() != var_de_destino.tipo_de_dado.clone() {
                        return Err(format!(
                            "ERRO SEMÂNTICO: O operador no segundo parâmetro não é compatível com {:?}!",
                            var_de_destino.tipo_de_dado.clone(),
                        ));
                    }
                } else {
                    return Err(String::from("ERRO NO COMPILADOR: Inconsistência em Semantico::modificar_variavel()!"));
                }

                parametro
            }
            None => {
                return Err(String::from("ERRO NO COMPILADOR: Inconsistência em Semantico::modificar_variavel()!"));
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

    fn gerar_inz(&mut self) -> Result<(), String> {
        if let Some(parametro) = self.pilha_de_parametros_condicionais.pop() {
            self.llvm.gerar_inz(parametro);
            return Ok(());
        } else {
            return Err(String::from("ERRO NO COMPILADOR: Inconsistência em Semantico::gerar_inz()!"));
        }
    }

    fn gerar_wnz(&mut self) -> Result<(), String> {
        if let Some(parametro) = self.pilha_de_parametros_condicionais.pop() {
            self.llvm.gerar_wnz(parametro);
            return Ok(());
        } else {
            return Err(String::from("ERRO NO COMPILADOR: Inconsistência em Semantico::gerar_wnz()!"));
        }
    }

    fn gerar_chamada_de_bloco(&mut self) -> Result<(), String> {
        if let Some(Tokens::IdDeBloco(bloco)) = self.pilha_de_tokens.pop() {
            if self.blocos_de_codigos.get(bloco.as_str()).is_none() {
                self.blocos_de_codigos.insert(bloco.to_string(), false);
            }

            self.llvm.gerar_chamada_de_bloco(ParametroLLVM::instanciar(
                bloco.as_str(),
                TipoDeParametroLLVM::Bloco,
                TipoDeDado::Undefined,
            ));
            
            return Ok(());
        } else {
            return Err(String::from("ERRO NO COMPILADOR: Inconsistência em Semantico::gerar_chamada_de_bloco()!"));
        }
    }

    fn empilhar_parametro_de_comparacao(&mut self) -> Result<(), String> {
        let token = self.pilha_de_tokens.pop();

        let parametro: ParametroLLVM = match token.clone() {
            Some(Tokens::Numero(numero)) => {
                ParametroLLVM::instanciar(
                    numero.as_str(),
                    TipoDeParametroLLVM::Numero,
                    TipoDeDado::Undefined,
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
                    return Err(String::from("ERRO NO COMPILADOR: Inconsistência em Semantico::empilhar_parametro_de_comparacao()!"));
                }
            }
        };

        self.pilha_de_parametros_condicionais.push(parametro);

        Ok(())
    }

    fn obter_operando2_de_operacao(&mut self) -> Result<(), String> {
        let token = self.pilha_de_tokens.pop();

        let parametro: ParametroLLVM = match token.clone() {
            Some(Tokens::Numero(numero)) => {
                ParametroLLVM::instanciar(
                    numero.as_str(),
                    TipoDeParametroLLVM::Numero,
                    TipoDeDado::Undefined,
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
                    return Err(String::from("ERRO NO COMPILADOR: Inconsistência em Semantico::empilhar_parametro_de_comparacao()!"));
                }
            }
        };

        self.retorno_de_operador = Some(parametro);

        Ok(())
    }

    fn empilhar_parametro_do_set(&mut self) -> Result<(), String> {
        let token = self.pilha_de_tokens.pop();

        let parametro: ParametroLLVM = match token.clone() {
            Some(Tokens::Numero(numero)) => {
                ParametroLLVM::instanciar(
                    numero.as_str(),
                    TipoDeParametroLLVM::Numero,
                    TipoDeDado::Undefined,
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

    fn gerar_operacao(&mut self) -> Result<(), String> {
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
                "A" => {
                    Operador::A
                }
                "B" => {
                    Operador::B
                }
                "AE" => {
                    Operador::AE
                }
                "BE" => {
                    Operador::BE
                }
                "E" => {
                    Operador::E
                }
                _ => {
                    return Err(String::from("ERRO NO COMPILADOR: Inconsistência em Semantico::gerar_operacao():5!"));
                }
            };

            if operador == Operador::NOT {
                let id_operando1: String = match self.pilha_de_tokens.pop() {
                    Some(Tokens::IdDeVariavel(id)) => {
                        id.to_string()
                    }
                    _ => {
                        return Err(String::from(
                            "ERRO NO COMPILADOR: Inconsistência em Semantico::modificar_variavel()!"
                        ));
                    }
                };
        
                let operando1: ParametroLLVM = match self.variaveis_globais.get(id_operando1.as_str()) {
                    Some(var) => {
                        var.clone()
                    },
                    None => {
                        return Err(format!(
                            "ERRO SEMÂNTICO: Variável @{} não declarada!",
                            id_operando1,
                        ));
                    }
                };

                let retorno = self.llvm.gerar_operacao(operando1, operador, None);

                self.retorno_de_operador = Some(retorno);
            } else {
                let id_operando1: String = match self.pilha_de_tokens.pop() {
                    Some(Tokens::IdDeVariavel(id)) => {
                        id.to_string()
                    }
                    _ => {
                        return Err(String::from(
                            "ERRO NO COMPILADOR: Inconsistência em Semantico::modificar_variavel()!"
                        ));
                    }
                };
        
                let operando1: ParametroLLVM = match self.variaveis_globais.get(id_operando1.as_str()) {
                    Some(var) => {
                        var.clone()
                    },
                    None => {
                        return Err(format!(
                            "ERRO SEMÂNTICO: Variável @{} não declarada!",
                            id_operando1,
                        ));
                    }
                };

                if let Some(operando2) = self.retorno_de_operador.clone() {

                    if operando2.tipo_de_parametro.clone() == TipoDeParametroLLVM::Numero {
                        match Semantico::numero_compativel(
                            operando1.tipo_de_dado.clone(),
                            operando2.parametro.as_str(),
                        ) {
                            Ok(compativel) => {
                                if !compativel {
                                    return Err(format!(
                                        "ERRO SEMÂNTICO: O número '{}' não é compatível com {:?}!",
                                        operando2.parametro,
                                        operando1.tipo_de_dado.clone(),
                                    ));
                                }

                                let numero = ParametroLLVM::instanciar(
                                    operando2.parametro.as_str(),
                                    TipoDeParametroLLVM::Numero,
                                    operando1.tipo_de_dado.clone(),
                                );

                                let retorno = self.llvm.gerar_operacao(operando1, operador, Some(numero));
                                self.retorno_de_operador = Some(retorno);
                            }
                            Err(erro) => return Err(erro),
                        }
                    } else if operando2.tipo_de_parametro.clone() == TipoDeParametroLLVM::VariavelGlobal {
                        if operando2.tipo_de_dado.clone() != operando1.tipo_de_dado.clone() {
                            return Err(format!(
                                "ERRO SEMÂNTICO: A variável {} não é compatível com {:?}!",
                                operando2.parametro,
                                operando1.tipo_de_dado.clone(),
                            ));
                        }

                        let retorno = self.llvm.gerar_operacao(operando1, operador, Some(operando2));
                        self.retorno_de_operador = Some(retorno);
                    } else if operando2.tipo_de_parametro.clone() == TipoDeParametroLLVM::VariavelTemporaria {
                        if operando2.tipo_de_dado.clone() != operando1.tipo_de_dado.clone() {
                            return Err(format!(
                                "ERRO SEMÂNTICO: O operador no segundo parâmetro não é compatível com {:?}!",
                                operando1.tipo_de_dado.clone(),
                            ));
                        }

                        let retorno = self.llvm.gerar_operacao(operando1, operador, Some(operando2));
                        self.retorno_de_operador = Some(retorno);
                    } else {
                        return Err(String::from("ERRO NO COMPILADOR: Inconsistência em Semantico::gerar_operacao().3!"));
                    }
                } else {
                    return Err(String::from("ERRO NO COMPILADOR: Inconsistência em Semantico::gerar_operacao().2!"));
                }
            }
            return Ok(());
        } else {
            return Err(String::from("ERRO NO COMPILADOR: Inconsistência em Semantico::gerar_operacao().1!"));
        }
    }

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