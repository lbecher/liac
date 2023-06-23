use crate::comum::*;

#[derive(Debug, Clone)]
pub struct LLVM {
    llvm_ir: String,
    llvm_ir_temp: String,
    contador_de_variaveis_temporarias: usize,
    contador_de_strings: usize,
}

impl LLVM {
    pub fn inicializar() -> Self {
        Self {
            llvm_ir: String::new(),
            llvm_ir_temp: String::new(),
            contador_de_variaveis_temporarias: 1,
            contador_de_strings: 1,
        }
    }

    fn adicionar_comando_llvm(
        &mut self,
        opcoes: OpcoesDoComando,
    ) {
        match opcoes.comando {
            Comando::DeclararString => {
                // @.str.1 = private unnamed_addr constant [4 x i8] c"%s\0A\00", align 1
                self.llvm_ir = format!(
                    "{1} = private unnamed_addr constant [{2} x i8] c\"{3}\\0A\\00\", align 1\n{0}",
                    self.llvm_ir,
                    opcoes.destino,
                    opcoes.parametros[0].len(),
                    opcoes.parametros[0],
                );
            }
            Comando::DeclararVarGlobal => {
                // @ch = dso_local global i8 0, align 1
                self.llvm_ir = format!(
                    "{1} = dso_local global i{2} {3}, align {4}\n{0}",
                    self.llvm_ir,
                    opcoes.destino,
                    opcoes.tamanho,
                    opcoes.parametros[0],
                    opcoes.alinhamento,
                );
            }
            Comando::LerVarGlobal => {
                if opcoes.tamanho != 8 && opcoes.alinhamento != 8 {
                    // %6 = load i8, i8* @ch, align 1
                    self.llvm_ir_temp = format!(
                        "{0}  {1} = load i{2}, i{2}* {3}, align {4}\n",
                        self.llvm_ir_temp,
                        opcoes.destino,
                        opcoes.tamanho,
                        opcoes.parametros[0],
                        opcoes.alinhamento,
                    );
                } else {
                    // ponteiro de string
                    // %1 = load i8*, i8** @st, align 8
                    self.llvm_ir_temp = format!(
                        "{0}  {1} = load i{2}, i{2}** {3}, align {4}\n",
                        self.llvm_ir_temp,
                        opcoes.destino,
                        opcoes.tamanho,
                        opcoes.parametros[0],
                        opcoes.alinhamento,
                    );
                }
            }
            Comando::GravarVarGlobal => {
                // store i32 %17, i32* @z, align 4
                self.llvm_ir_temp = format!(
                    "{0}  store i{2} {3}, i{2}* {1}, align {4}\n",
                    self.llvm_ir_temp,
                    opcoes.destino,
                    opcoes.tamanho,
                    opcoes.parametros[0],
                    opcoes.alinhamento,
                );
            }
            Comando::Somar => {
                // %11 = add nsw i32 %9, %10
                self.llvm_ir_temp = format!(
                    "{0}  {1} = add {5}i{2} {3}, {4}\n",
                    self.llvm_ir_temp,
                    opcoes.destino,
                    opcoes.tamanho,
                    opcoes.parametros[0],
                    opcoes.parametros[1],
                    if opcoes.sinalizado { "nsw " } else { "" },
                );
            }
            Comando::Subtrair => {
                // %11 = sub nsw i32 %9, %10
                self.llvm_ir_temp = format!(
                    "{0}  {1} = sub {5}i{2} {3}, {4}\n",
                    self.llvm_ir_temp,
                    opcoes.destino,
                    opcoes.tamanho,
                    opcoes.parametros[0],
                    opcoes.parametros[1],
                    if opcoes.sinalizado { "nsw " } else { "" },
                );
            }
            _ => {

            }
        }
    }

    pub fn criar_funcao_main(&mut self) {
        self.llvm_ir = format!(
            "{}\ndefine dso_local i32 @main() #0 {{\n{}  ret i32 0\n}}\n",
            self.llvm_ir,
            self.llvm_ir_temp,
        );
        self.llvm_ir_temp = String::new();
    }

    pub fn criar_funcao_de_bloco(&mut self, nome: &str) {
        self.llvm_ir = format!(
            "{}\ndefine dso_local i32 @bloc_{} #0 {{\n{}  ret i32 0\n}}\n",
            self.llvm_ir,
            nome,
            self.llvm_ir_temp,
        );
        self.llvm_ir_temp = String::new();
    }

    pub fn declarar_string(&mut self, string: &str) {
        let destino = format!(".str.{}", self.contador_de_strings);
        self.contador_de_strings += 1;
        let opcoes = OpcoesDoComando::instanciar(
            Comando::DeclararVarGlobal,
            TiposDeDado::String,
            destino.as_str(),
            [string, ""],
        );
        self.adicionar_comando_llvm(opcoes);
    }

    pub fn declarar_var_global(&mut self, nome: &str, tipo: TiposDeDado) {
        let opcoes = OpcoesDoComando::instanciar(
            Comando::DeclararVarGlobal,
            tipo.clone(),
            format!("@{}", nome).as_str(),
            [
                if TiposDeDado::String == tipo { "null" } else { "0" },
                "",
            ],
        );
        self.adicionar_comando_llvm(opcoes);
    }

    pub fn modificar_variavel(&mut self, nome: &str, tipo: TiposDeDado, parametro: ParametroGenerico) {
        let variavel = format!("@{}", nome);
        let mut opcoes_do_comando: OpcoesDoComando;
        match parametro {
            ParametroGenerico::Imediato(imediato) => {
                opcoes_do_comando = OpcoesDoComando::instanciar(
                    Comando::GravarVarGlobal,
                    tipo.clone(),
                    variavel.as_str(),
                    [imediato.as_str(), ""],
                );
                self.adicionar_comando_llvm(opcoes_do_comando);
            }
            ParametroGenerico::VariavelTemporaria(variavel_temporaria) => {
                opcoes_do_comando = OpcoesDoComando::instanciar(
                    Comando::GravarVarGlobal,
                    tipo.clone(),
                    variavel.as_str(),
                    [variavel_temporaria.nome.as_str(), ""],
                );
                self.adicionar_comando_llvm(opcoes_do_comando);
            }
            ParametroGenerico::VariavelGlobal(variavel_global) => {
                let retorno_parametro = format!("%{}", self.contador_de_variaveis_temporarias);
                opcoes_do_comando = OpcoesDoComando::instanciar(
                    Comando::LerVarGlobal,
                    tipo.clone(),
                    retorno_parametro.as_str(),
                    [format!("@{}", variavel_global).as_str(), ""],
                );
                self.adicionar_comando_llvm(opcoes_do_comando);
                self.contador_de_variaveis_temporarias += 1;
                
                opcoes_do_comando = OpcoesDoComando::instanciar(
                    Comando::GravarVarGlobal,
                    tipo.clone(),
                    variavel.as_str(),
                    [retorno_parametro.as_str(), ""],
                );
                self.adicionar_comando_llvm(opcoes_do_comando);
            }
        }
    }

    pub fn adicionar_operacao(
        &mut self,
        operando1: &str,
        operador: Operador,
        operando2: Option<ParametroGenerico>,
        tipo: TiposDeDado,
    ) -> (String, TiposDeDado) {
        let mut segundo_parametro_precisa_gravar = false;
        
        let primeiro_parametro = format!("@{}", operando1);
        let segundo_parametro = match operando2 {
            Some(algo) => {
                match algo {
                    ParametroGenerico::Imediato(im) => {
                        im
                    }
                    ParametroGenerico::VariavelGlobal(vg) => {
                        segundo_parametro_precisa_gravar = true;
                        format!("@{}", vg)
                    }
                    ParametroGenerico::VariavelTemporaria(vt) => {
                        vt.nome
                    }
                }
            }
            None => {
                String::new()
            }
        };

        let mut opcoes_do_comando: OpcoesDoComando;

        /* escreve o comando de leitura do primeiro argumento */
        let retorno_primeiro_parametro = format!("%{}", self.contador_de_variaveis_temporarias);
        opcoes_do_comando = OpcoesDoComando::instanciar(
            Comando::LerVarGlobal,
            tipo.clone(),
            retorno_primeiro_parametro.as_str(),
            [primeiro_parametro.as_str(), ""],
        );
        self.adicionar_comando_llvm(opcoes_do_comando);
        self.contador_de_variaveis_temporarias += 1;

        /* trata os demais comandos */
        let retorno_final = match operador {
            Operador::ADD => {
                /* escreve o comando de leitura do segundo argumento, se houver */
                let retorno_segundo_parametro: String;
                if segundo_parametro_precisa_gravar {
                    retorno_segundo_parametro = format!("%{}", self.contador_de_variaveis_temporarias);
                    opcoes_do_comando = OpcoesDoComando::instanciar(
                        Comando::LerVarGlobal,
                        tipo.clone(),
                        retorno_segundo_parametro.as_str(),
                        [segundo_parametro.as_str(), ""],
                    );
                    self.adicionar_comando_llvm(opcoes_do_comando);
                    self.contador_de_variaveis_temporarias += 1;
                } else {
                    retorno_segundo_parametro = segundo_parametro;
                }

                /* escreve o comando da operação */
                let retorno_da_soma = format!("%{}", self.contador_de_variaveis_temporarias);
                opcoes_do_comando = OpcoesDoComando::instanciar(
                    Comando::Somar,
                    tipo.clone(),
                    retorno_da_soma.as_str(),
                    [retorno_primeiro_parametro.as_str(), retorno_segundo_parametro.as_str()],
                );
                self.adicionar_comando_llvm(opcoes_do_comando);
                self.contador_de_variaveis_temporarias += 1;

                retorno_da_soma
            }
            Operador::SUB => {
                /* escreve o comando de leitura do segundo argumento, se houver */
                let retorno_segundo_parametro: String;
                if segundo_parametro_precisa_gravar {
                    retorno_segundo_parametro = format!("%{}", self.contador_de_variaveis_temporarias);
                    opcoes_do_comando = OpcoesDoComando::instanciar(
                        Comando::LerVarGlobal,
                        tipo.clone(),
                        retorno_segundo_parametro.as_str(),
                        [segundo_parametro.as_str(), ""],
                    );
                    self.adicionar_comando_llvm(opcoes_do_comando);
                    self.contador_de_variaveis_temporarias += 1;
                } else {
                    retorno_segundo_parametro = segundo_parametro;
                }

                /* escreve o comando da operação */
                let retorno_da_soma = format!("%{}", self.contador_de_variaveis_temporarias);
                opcoes_do_comando = OpcoesDoComando::instanciar(
                    Comando::Subtrair,
                    tipo.clone(),
                    retorno_da_soma.as_str(),
                    [retorno_primeiro_parametro.as_str(), retorno_segundo_parametro.as_str()],
                );
                self.adicionar_comando_llvm(opcoes_do_comando);
                self.contador_de_variaveis_temporarias += 1;

                retorno_da_soma
            }
            _ => {
                String::new()
            }
        };

        /* retorna variável temporária da operação */
        return (
            retorno_final,
            tipo.clone(),
        );
    }

    pub fn obter_llvm_ir(&self) -> String {
        self.llvm_ir.to_string()
    }
}