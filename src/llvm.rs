use liac::*;

#[derive(Debug, Clone)]
pub struct LLVM {
    codigo_llvm: String,
    bloco_basico: String,
    bloco_basico_niveis: Vec<String>,
    bloco_de_funcao: String,
    contador_geral: usize,
    contador_de_strings: usize,
    
}

impl LLVM {
    pub fn inicializar() -> Self {
        let mut codigo_llvm = String::new();

        codigo_llvm += "\ndeclare i32 @printf(i8* noundef, ...) #1";
        codigo_llvm += "\ndeclare i32 @__isoc99_scanf(i8* noundef, ...) #1\n";

        Self {
            codigo_llvm,
            bloco_basico: String::new(),
            bloco_basico_niveis: Vec::new(),
            bloco_de_funcao: String::new(),
            contador_geral: 1,
            contador_de_strings: 1,
        }
    }

    pub fn obter_codigo_llvm(&self) -> String {
        self.codigo_llvm.to_string()
    }

    pub fn incrementar_contador_geral(&mut self) {
        self.contador_geral += 1;
    }

    fn incrementar_contador_de_strings(&mut self) {
        self.contador_de_strings += 1;
    }

    fn criar_var_temporaria(&mut self, tipo_de_dado: TipoDeDado) -> ParametroLLVM {
        let var_temporaria = ParametroLLVM::instanciar(
            format!("{}", self.contador_geral).as_str(),
            TipoDeParametroLLVM::VariavelTemporaria,
            tipo_de_dado,
        );
        self.incrementar_contador_geral();
        return var_temporaria;
    }

    pub fn gerar_bloco_main(&mut self) {
        let mut codigo_llvm = String::new();

        codigo_llvm += "\ndefine dso_local i32 @main() #0 {\n";
        codigo_llvm += self.bloco_de_funcao.as_str();

        if codigo_llvm.contains("$$$$") {
            codigo_llvm += "  br label %$$$$\n\n$$$$:\n";
        }

        codigo_llvm += "  ret i32 0\n";
        codigo_llvm += "}\n";

        codigo_llvm = codigo_llvm.replace(
            "$$$$",
            format!("{}", self.contador_geral).as_str()
        );

        self.codigo_llvm += codigo_llvm.as_str();
        self.bloco_de_funcao = String::new();
        self.contador_geral = 1;
    }

    pub fn gerar_bloco(&mut self, nome: &str) {
        let mut codigo_llvm = String::new();

        codigo_llvm += "\ndefine dso_local void @bloc_";
        codigo_llvm += nome;
        codigo_llvm += "() #0 {\n";
        codigo_llvm += self.bloco_de_funcao.as_str();

        if codigo_llvm.contains("$$$$") {
            codigo_llvm += "  br label %$$$$\n\n$$$$:\n";
        }

        codigo_llvm += "  ret void\n";
        codigo_llvm += "}\n";

        codigo_llvm = codigo_llvm.replace(
            "$$$$",
            format!("{}", self.contador_geral).as_str()
        );

        self.codigo_llvm += codigo_llvm.as_str();
        self.bloco_de_funcao = String::new();
        self.contador_geral = 1;
    }

    pub fn bloco_basico_para_bloco_de_funcao(&mut self) {
        self.bloco_de_funcao += self.bloco_basico.as_str();
        self.bloco_basico = String::new();
    }

    fn gerar_comando_llvm(
        &mut self,
        comando_llvm: ComandoLLVM,
    ) {
        match comando_llvm.comando {
            Comandos::DeclararString => {
                self.codigo_llvm = format!(
                    "{1} = private unnamed_addr constant [{2} x i8] {3}, align 1\n{0}",
                    self.codigo_llvm,
                    comando_llvm.destino.unwrap().parametro,
                    comando_llvm.parametros[0].tamanho_do_array.unwrap(),
                    comando_llvm.parametros[0].parametro,
                );
            }
            Comandos::DeclararVariavel => {
                let destino = comando_llvm.destino.unwrap();
                self.codigo_llvm = format!(
                    "{1} = dso_local global i{2} {3}, align {4}\n{0}",
                    self.codigo_llvm,
                    destino.parametro,
                    destino.tamanho,
                    if destino.tipo_de_dado == TipoDeDado::String { "null" } else { "0" },
                    destino.alinhamento,
                );
            }
            Comandos::ChamarBloco => {
                self.bloco_basico = format!(
                    "{0}  call void {1}\n",
                    self.bloco_basico,
                    comando_llvm.parametros[0].parametro,
                );
            }
            Comandos::LerVariavel => {
                if comando_llvm.parametros[0].tipo_de_dado != TipoDeDado::String {
                    let destino = comando_llvm.destino.unwrap();
                    self.bloco_basico = format!(
                        "{0}  {1} = load i{2}, i{3}* {4}, align {5}\n",
                        self.bloco_basico,
                        destino.parametro,
                        destino.tamanho,
                        comando_llvm.parametros[0].tamanho,
                        comando_llvm.parametros[0].parametro,
                        destino.alinhamento,
                    );
                } else {
                    // ponteiro de string
                    self.bloco_basico = format!(
                        "{0}  {1} = load i8, i8** {2}, align 8\n",
                        self.bloco_basico,
                        comando_llvm.destino.unwrap().parametro,
                        comando_llvm.parametros[0].parametro,
                    );
                }
            }
            Comandos::GravarVariavel => {
                let destino = comando_llvm.destino.unwrap();
                self.bloco_basico = format!(
                    "{0}  store i{1} {2}, i{3}* {4}, align {5}\n",
                    self.bloco_basico,
                    comando_llvm.parametros[0].tamanho,
                    comando_llvm.parametros[0].parametro,
                    destino.tamanho,
                    destino.parametro,
                    destino.alinhamento,
                );
            }
            Comandos::Print => {
                let mut parametros = comando_llvm.parametros.to_vec();
                parametros.remove(0);

                let mut parametros_scanf = String::new();

                for parametro in parametros {
                    if parametro.tipo_de_parametro.clone() == TipoDeParametroLLVM::VariavelGlobal {
                        let var_temporaria = self.ler_variavel_global(parametro.clone());
                        parametros_scanf += format!(", i{} noundef {}", var_temporaria.tamanho, var_temporaria.parametro).as_str();
                    } else {
                        parametros_scanf += format!(", i{} noundef {}", parametro.tamanho, parametro.parametro).as_str();
                    }
                }

                let retorno = self.criar_var_temporaria(TipoDeDado::Int32);

                self.bloco_basico = format!(
                    "{0}  {1} = call i32 (i8*, ...) @printf(i8* noundef getelementptr inbounds ([{2} x i8], [{2} x i8]* {3}, i64 0, i64 0){4})\n",
                    self.bloco_basico,
                    retorno.parametro,
                    comando_llvm.parametros[0].tamanho_do_array.unwrap(),
                    comando_llvm.parametros[0].parametro,
                    parametros_scanf,
                );
            }
            Comandos::Scan => {
                let mut parametros = comando_llvm.parametros.to_vec();
                parametros.remove(0);

                let mut parametros_scanf = String::new();

                for parametro in parametros {
                    parametros_scanf += format!(", i{}* noundef {}", parametro.tamanho, parametro.parametro).as_str();
                }

                let retorno = self.criar_var_temporaria(TipoDeDado::Int32);

                self.bloco_basico = format!(
                    "{0}  {1} = call i32 (i8*, ...) @__isoc99_scanf(i8* noundef getelementptr inbounds ([{2} x i8], [{2} x i8]* {3}, i64 0, i64 0){4})\n",
                    self.bloco_basico,
                    retorno.parametro,
                    comando_llvm.parametros[0].tamanho_do_array.unwrap(),
                    comando_llvm.parametros[0].parametro,
                    parametros_scanf,
                );
            }
            Comandos::Somar => {
                let destino = comando_llvm.destino.unwrap();
                self.bloco_basico = format!(
                    "{0}  {1} = add {2}i{3} {4}, {5}\n",
                    self.bloco_basico,
                    destino.parametro,
                    if destino.sinalizado { "nsw " } else { "" },
                    destino.tamanho,
                    comando_llvm.parametros[0].parametro,
                    comando_llvm.parametros[1].parametro,
                );
            }
            Comandos::Subtrair => {
                let destino = comando_llvm.destino.unwrap();
                self.bloco_basico = format!(
                    "{0}  {1} = sub {2}i{3} {4}, {5}\n",
                    self.bloco_basico,
                    destino.parametro,
                    if destino.sinalizado { "nsw " } else { "" },
                    destino.tamanho,
                    comando_llvm.parametros[0].parametro,
                    comando_llvm.parametros[1].parametro,
                );
            }
            Comandos::Multiplicar => {
                let destino = comando_llvm.destino.unwrap();
                self.bloco_basico = format!(
                    "{0}  {1} = mul {2}i{3} {4}, {5}\n",
                    self.bloco_basico,
                    destino.parametro,
                    if destino.sinalizado { "nsw " } else { "" },
                    destino.tamanho,
                    comando_llvm.parametros[0].parametro,
                    comando_llvm.parametros[1].parametro,
                );
            }
            Comandos::Dividir => {
                let destino = comando_llvm.destino.unwrap();
                self.bloco_basico = format!(
                    "{0}  {1} = {2} i{3} {4}, {5}\n",
                    self.bloco_basico,
                    destino.parametro,
                    if destino.sinalizado { "sdiv" } else { "udiv" },
                    destino.tamanho,
                    comando_llvm.parametros[0].parametro,
                    comando_llvm.parametros[1].parametro,
                );
            }
            Comandos::RestoDaDivisao => {
                let destino = comando_llvm.destino.unwrap();
                self.bloco_basico = format!(
                    "{0}  {1} = {2} i{3} {4}, {5}\n",
                    self.bloco_basico,
                    destino.parametro,
                    if destino.sinalizado { "srem" } else { "urem" },
                    destino.tamanho,
                    comando_llvm.parametros[0].parametro,
                    comando_llvm.parametros[1].parametro,
                );
            }
            Comandos::ConverterBooleano => {
                let destino = comando_llvm.destino.unwrap();
                self.bloco_basico = format!(
                    "{0}  {1} = zext i1 {2} to i{3}\n",
                    self.bloco_basico,
                    destino.parametro,
                    comando_llvm.parametros[0].parametro,
                    destino.tamanho,
                );
            }
            Comandos::And => {
                let destino = comando_llvm.destino.unwrap();
                self.bloco_basico = format!(
                    "{0}  {1} = and i{2} {3}, {4}\n",
                    self.bloco_basico,
                    destino.parametro,
                    destino.tamanho,
                    comando_llvm.parametros[0].parametro,
                    comando_llvm.parametros[1].parametro,
                );
            }
            Comandos::Or => {
                let destino = comando_llvm.destino.unwrap();
                self.bloco_basico = format!(
                    "{0}  {1} = or i{2} {3}, {4}\n",
                    self.bloco_basico,
                    destino.parametro,
                    destino.tamanho,
                    comando_llvm.parametros[0].parametro,
                    comando_llvm.parametros[1].parametro,
                );
            }
            Comandos::Not => {
                let destino = comando_llvm.destino.unwrap();
                self.bloco_basico = format!(
                    "{0}  {1} = xor i{2} {3}, -1\n",
                    self.bloco_basico,
                    destino.parametro,
                    destino.tamanho,
                    comando_llvm.parametros[0].parametro,
                );
            }
            Comandos::Igual => {
                self.bloco_basico = format!(
                    "{0}  {1} = icmp eq i{2} {3}, {4}\n",
                    self.bloco_basico,
                    comando_llvm.destino.unwrap().parametro,
                    comando_llvm.parametros[0].tamanho,
                    comando_llvm.parametros[0].parametro,
                    comando_llvm.parametros[1].parametro,
                );
            }
            Comandos::Menor => {
                self.bloco_basico = format!(
                    "{0}  {1} = icmp {5} i{2} {3}, {4}\n",
                    self.bloco_basico,
                    comando_llvm.destino.unwrap().parametro,
                    comando_llvm.parametros[0].tamanho,
                    comando_llvm.parametros[0].parametro,
                    comando_llvm.parametros[1].parametro,
                    if comando_llvm.parametros[0].sinalizado { "slt" } else { "ult" },
                );
            }
            Comandos::Maior => {
                self.bloco_basico = format!(
                    "{0}  {1} = icmp {5} i{2} {3}, {4}\n",
                    self.bloco_basico,
                    comando_llvm.destino.unwrap().parametro,
                    comando_llvm.parametros[0].tamanho,
                    comando_llvm.parametros[0].parametro,
                    comando_llvm.parametros[1].parametro,
                    if comando_llvm.parametros[0].sinalizado { "sgt" } else { "ugt" },
                );
            }
            Comandos::MenorIgual => {
                self.bloco_basico = format!(
                    "{0}  {1} = icmp {5} i{2} {3}, {4}\n",
                    self.bloco_basico,
                    comando_llvm.destino.unwrap().parametro,
                    comando_llvm.parametros[0].tamanho,
                    comando_llvm.parametros[0].parametro,
                    comando_llvm.parametros[1].parametro,
                    if comando_llvm.parametros[0].sinalizado { "sle" } else { "ule" },
                );
            }
            Comandos::MaiorIgual => {
                self.bloco_basico = format!(
                    "{0}  {1} = icmp {5} i{2} {3}, {4}\n",
                    self.bloco_basico,
                    comando_llvm.destino.unwrap().parametro,
                    comando_llvm.parametros[0].tamanho,
                    comando_llvm.parametros[0].parametro,
                    comando_llvm.parametros[1].parametro,
                    if comando_llvm.parametros[0].sinalizado { "sge" } else { "uge" },
                );
            }
            Comandos::Diferente => {
                self.bloco_basico = format!(
                    "{0}  {1} = icmp {5} i{2} {3}, {4}\n",
                    self.bloco_basico,
                    comando_llvm.destino.unwrap().parametro,
                    comando_llvm.parametros[0].tamanho,
                    comando_llvm.parametros[0].parametro,
                    comando_llvm.parametros[1].parametro,
                    "ne",
                );
            }
        }
    }

    pub fn declarar_variavel_global(&mut self, nome: &str, tipo: TipoDeDado) {
        let comando_llvm = ComandoLLVM {
            comando: Comandos::DeclararVariavel,
            destino: Some(
                ParametroLLVM::instanciar(
                    nome,
                    TipoDeParametroLLVM::VariavelGlobal,
                    tipo,
                )
            ),
            parametros: vec![],
        };
        self.gerar_comando_llvm(comando_llvm);
    }

    pub fn declarar_string(&mut self, string: ParametroLLVM) -> ParametroLLVM {
        let label_da_string = format!(".str.{}", self.contador_de_strings);

        let  mut var_da_string = ParametroLLVM::instanciar(
            label_da_string.as_str(),
            TipoDeParametroLLVM::VariavelGlobal,
            TipoDeDado::String,
        );
        var_da_string.setar_tamanho_do_array(string.tamanho_do_array.clone());

        let comando_llvm = ComandoLLVM {
            comando: Comandos::DeclararString,
            destino: Some(
                var_da_string.clone(),
            ),
            parametros: vec![
                string
            ],
        };
        self.gerar_comando_llvm(comando_llvm);

        self.incrementar_contador_de_strings();

        return var_da_string;
    }

    fn ler_variavel_global(&mut self, var_global: ParametroLLVM) -> ParametroLLVM {
        let var_temporaria = self.criar_var_temporaria(var_global.tipo_de_dado.clone());

        let comando = ComandoLLVM {
            comando: Comandos::LerVariavel,
            destino: Some(var_temporaria.clone()),
            parametros: vec![
                var_global
            ]
        };
        self.gerar_comando_llvm(comando);

        return var_temporaria;
    }

    pub fn gerar_retorno(&mut self) {
        self.bloco_basico = format!(
            "{0}  br label %$$$$\n\n{1}:\n",
            self.bloco_basico,
            self.contador_geral,
        );

        self.incrementar_contador_geral();
    }

    
    pub fn gerar_wnz(
        &mut self,
        parametro: ParametroLLVM,
    ) {
        let min_max = obter_maximo_minimo(&self.bloco_basico);
        let bloco_basico: String;

        if let Some((min, _)) = min_max {
            bloco_basico = substituir_com_decremento(
                self.bloco_basico.as_str(), 
                min,
            );
            self.contador_geral = min;
        } else {
            bloco_basico = self.bloco_basico.to_string();
        }

        self.bloco_basico = String::new();
        
        let label_um = self.criar_var_temporaria(TipoDeDado::Undefined).parametro.to_string();
        let label_um_numero = label_um.to_string();
        let (_, label_um_numero) = label_um_numero.split_at(1);

        self.bloco_basico += format!("  br label {0}\n\n{1}:\n",
            label_um.to_string(),
            label_um_numero.to_string(),
        ).as_str();

        let comparacao: ParametroLLVM;

        if parametro.tipo_de_parametro.clone() == TipoDeParametroLLVM::VariavelGlobal {
            comparacao = self.ler_variavel_global(parametro);
        } else {
            comparacao = parametro;
        }

        let zero = ParametroLLVM::instanciar(
            "0",
            TipoDeParametroLLVM::Numero,
            comparacao.tipo_de_dado.clone(),
        );

        let var_de_destino = self.criar_var_temporaria(
            comparacao.tipo_de_dado.clone()
        );

        let comando = ComandoLLVM {
            comando: Comandos::Diferente,
            destino: Some(var_de_destino.clone()),
            parametros: vec![
                comparacao,
                zero,
            ],
        };
        self.gerar_comando_llvm(comando);

        let label_dois = self.criar_var_temporaria(TipoDeDado::Undefined).parametro.to_string();
        let label_dois_numero = label_dois.to_string();
        let (_, label_dois_numero) = label_dois_numero.split_at(1);

        let bloco_basico = substituir_com_incremento(&bloco_basico, self.contador_geral);

        if let Some((min, max)) = min_max {
            self.contador_geral += (max - min) + 1;
        }
        
        self.bloco_basico += format!(
            "  br i1 {0}, label {1}, label %$$$$\n\n{2}:\n{3}  br label {4}\n\n$$$$:\n",
            var_de_destino.parametro.as_str(),
            label_dois.to_string(),
            label_dois_numero.to_string(),
            bloco_basico,
            label_um.to_string(),
        ).as_str();
        
        let label_tres = self.criar_var_temporaria(TipoDeDado::Undefined).parametro.to_string();
        let label_tres_numero = label_tres.to_string();
        let (_, label_tres_numero) = label_tres_numero.split_at(1);

        self.bloco_basico = self.bloco_basico.replace(
            "$$$$",
            format!("{}", label_tres_numero).as_str()
        );
    }


    pub fn gerar_inz(
        &mut self,
        parametro: ParametroLLVM,
    ) {
        let min_max = obter_maximo_minimo(&self.bloco_basico);
        let bloco_basico: String;

        if let Some((min, _)) = min_max {
            bloco_basico = substituir_com_decremento(
                self.bloco_basico.as_str(), 
                min,
            );
            self.contador_geral = min;
        } else {
            bloco_basico = self.bloco_basico.to_string();
        }

        self.bloco_basico = String::new();

        let comparacao: ParametroLLVM;

        if parametro.tipo_de_parametro.clone() == TipoDeParametroLLVM::VariavelGlobal {
            comparacao = self.ler_variavel_global(parametro);
        } else {
            comparacao = parametro;
        }

        let zero = ParametroLLVM::instanciar(
            "0",
            TipoDeParametroLLVM::Numero,
            comparacao.tipo_de_dado.clone(),
        );

        let var_de_destino = self.criar_var_temporaria(
            comparacao.tipo_de_dado.clone()
        );

        let comando = ComandoLLVM {
            comando: Comandos::Diferente,
            destino: Some(var_de_destino.clone()),
            parametros: vec![
                comparacao,
                zero,
            ],
        };
        self.gerar_comando_llvm(comando);

        let label_um = self.criar_var_temporaria(TipoDeDado::Undefined).parametro.to_string();
        let label_um_numero = label_um.to_string();
        let (_, label_um_numero) = label_um_numero.split_at(1);

        let bloco_basico = substituir_com_incremento(&bloco_basico, self.contador_geral);

        if let Some((min, max)) = min_max {
            self.contador_geral += (max - min) + 1;
        }
        
        self.bloco_basico += format!(
            "  br i1 {0}, label {1}, label %$$$$\n\n{2}:\n{3}  br label {4}\n\n$$$$:\n",
            var_de_destino.parametro.as_str(),
            label_um.to_string(),
            label_um_numero.to_string(),
            bloco_basico,
            label_um.to_string(),
        ).as_str();
        
        let label_dois = self.criar_var_temporaria(TipoDeDado::Undefined).parametro.to_string();
        let label_dois_numero = label_dois.to_string();
        let (_, label_dois_numero) = label_dois_numero.split_at(1);

        self.bloco_basico = self.bloco_basico.replace(
            "$$$$",
            format!("{}", label_dois_numero).as_str()
        );

        /*let mut incremento: usize = 1;
        let bloco_basico = self.bloco_basico.to_string();
        self.bloco_basico = String::new();

        let inicio = self.contador_geral;

        let comparacao: ParametroLLVM;

        if parametro.tipo_de_parametro.clone() == TipoDeParametroLLVM::VariavelGlobal {
            comparacao = self.ler_variavel_global(parametro);
            incremento += 1;
        } else {
            comparacao = parametro;
        }

        let zero = ParametroLLVM::instanciar(
            "0",
            TipoDeParametroLLVM::Numero,
            comparacao.tipo_de_dado.clone(),
        );

        let var_de_destino = self.criar_var_temporaria(comparacao.tipo_de_dado.clone());

        let comando = ComandoLLVM {
            comando: Comandos::Diferente,
            destino: Some(var_de_destino),
            parametros: vec![
                comparacao,
                zero,
            ],
        };
        self.gerar_comando_llvm(comando);

        /*let (bloco_basico, deslocamento) = substituir_com_incremento(
            bloco_basico.as_str(),
            incremento,
        );*/

        self.bloco_basico += format!(
            "  br i1 %{0}, label %{1}, label %$$$$\n\n{1}:\n{2}  br label %$$$$\n\n$$$$:\n",
            self.contador_geral - 1,
            self.contador_geral,
            bloco_basico,
        ).as_str();

        //self.contador_geral += deslocamento + 1;

        self.bloco_basico = self.bloco_basico.replace(
            "$$$$",
            format!("{}", self.contador_geral).as_str()
        );

        //self.incrementar_contador_geral();*/
    }

    pub fn gerar_print(
        &mut self,
        mascara: ParametroLLVM,
        parametros: Vec<ParametroLLVM>,
    ) {
        let mut parametros_completos: Vec<ParametroLLVM> = Vec::new();
        parametros_completos.push(mascara);
        parametros_completos.extend(parametros);

        let comando = ComandoLLVM {
            comando: Comandos::Print,
            destino: None,
            parametros: parametros_completos,
        };

        self.gerar_comando_llvm(comando);

        //self.bloco_basico_para_bloco_de_funcao();
    }

    pub fn gerar_scan(
        &mut self,
        mascara: ParametroLLVM,
        parametros: Vec<ParametroLLVM>,
    ) {
        let mut parametros_completos: Vec<ParametroLLVM> = Vec::new();
        parametros_completos.push(mascara);
        parametros_completos.extend(parametros);

        let comando = ComandoLLVM {
            comando: Comandos::Scan,
            destino: None,
            parametros: parametros_completos,
        };

        self.gerar_comando_llvm(comando);

        //self.bloco_basico_para_bloco_de_funcao();
    }

    pub fn gerar_atribuicao(
        &mut self,
        var_de_destino: ParametroLLVM,
        atribuicao: ParametroLLVM,
    ) {
        if atribuicao.tipo_de_parametro.clone() == TipoDeParametroLLVM::VariavelGlobal {
            let var_temporaria = self.criar_var_temporaria(atribuicao.tipo_de_dado.clone());

            let comando = ComandoLLVM {
                comando: Comandos::LerVariavel,
                destino: Some(var_temporaria.clone()),
                parametros: vec![
                    atribuicao
                ],
            };
            self.gerar_comando_llvm(comando);

            let comando = ComandoLLVM {
                comando: Comandos::GravarVariavel,
                destino: Some(var_de_destino),
                parametros: vec![
                    var_temporaria
                ],
            };
            self.gerar_comando_llvm(comando);
        } else {
            let comando = ComandoLLVM {
                comando: Comandos::GravarVariavel,
                destino: Some(var_de_destino),
                parametros: vec![
                    atribuicao
                ],
            };
            self.gerar_comando_llvm(comando);
        }

        //self.bloco_basico_para_bloco_de_funcao();
    }

    pub fn gerar_operacao(
        &mut self,
        operando1: ParametroLLVM,
        operador: Operador,
        operando2: Option<ParametroLLVM>,
    ) -> ParametroLLVM {
        let primeiro_parametro: ParametroLLVM;
        let segundo_parametro: Option<ParametroLLVM>;

        primeiro_parametro = self.ler_variavel_global(operando1);

        if let Some(op2) = operando2 {
            if op2.tipo_de_parametro == TipoDeParametroLLVM::VariavelGlobal {
                segundo_parametro = Some(self.ler_variavel_global(op2));
            } else {
                segundo_parametro = Some(op2);
            }
        } else {
            segundo_parametro = None;
        }

        let mut destino = self
            .criar_var_temporaria(primeiro_parametro.tipo_de_dado.clone());

        match operador {
            Operador::E => {
                let temp = destino;

                destino = self
                    .criar_var_temporaria(primeiro_parametro.tipo_de_dado.clone());

                let comando = ComandoLLVM {
                    comando: Comandos::Igual,
                    destino: Some(temp.clone()),
                    parametros: vec![
                        primeiro_parametro,
                        segundo_parametro.unwrap(),
                    ],
                };
                self.gerar_comando_llvm(comando);

                let comando = ComandoLLVM {
                    comando: Comandos::ConverterBooleano,
                    destino: Some(destino.clone()),
                    parametros: vec![
                        temp,
                    ],
                };
                self.gerar_comando_llvm(comando);
            }
            Operador::A => {
                let temp = destino;

                destino = self
                    .criar_var_temporaria(primeiro_parametro.tipo_de_dado.clone());

                let comando = ComandoLLVM {
                    comando: Comandos::Maior,
                    destino: Some(temp.clone()),
                    parametros: vec![
                        primeiro_parametro,
                        segundo_parametro.unwrap(),
                    ],
                };
                self.gerar_comando_llvm(comando);

                let comando = ComandoLLVM {
                    comando: Comandos::ConverterBooleano,
                    destino: Some(destino.clone()),
                    parametros: vec![
                        temp,
                    ],
                };
                self.gerar_comando_llvm(comando);
            }
            Operador::B => {
                let temp = destino;

                destino = self
                    .criar_var_temporaria(primeiro_parametro.tipo_de_dado.clone());

                let comando = ComandoLLVM {
                    comando: Comandos::Menor,
                    destino: Some(temp.clone()),
                    parametros: vec![
                        primeiro_parametro,
                        segundo_parametro.unwrap(),
                    ],
                };
                self.gerar_comando_llvm(comando);

                let comando = ComandoLLVM {
                    comando: Comandos::ConverterBooleano,
                    destino: Some(destino.clone()),
                    parametros: vec![
                        temp,
                    ],
                };
                self.gerar_comando_llvm(comando);
            }
            Operador::AE => {
                let temp = destino;

                destino = self
                    .criar_var_temporaria(primeiro_parametro.tipo_de_dado.clone());

                let comando = ComandoLLVM {
                    comando: Comandos::MaiorIgual,
                    destino: Some(temp.clone()),
                    parametros: vec![
                        primeiro_parametro,
                        segundo_parametro.unwrap(),
                    ],
                };
                self.gerar_comando_llvm(comando);

                let comando = ComandoLLVM {
                    comando: Comandos::ConverterBooleano,
                    destino: Some(destino.clone()),
                    parametros: vec![
                        temp,
                    ],
                };
                self.gerar_comando_llvm(comando);
            }
            Operador::BE => {
                let temp = destino;

                destino = self
                    .criar_var_temporaria(primeiro_parametro.tipo_de_dado.clone());

                let comando = ComandoLLVM {
                    comando: Comandos::MenorIgual,
                    destino: Some(temp.clone()),
                    parametros: vec![
                        primeiro_parametro,
                        segundo_parametro.unwrap(),
                    ],
                };
                self.gerar_comando_llvm(comando);

                let comando = ComandoLLVM {
                    comando: Comandos::ConverterBooleano,
                    destino: Some(destino.clone()),
                    parametros: vec![
                        temp,
                    ],
                };
                self.gerar_comando_llvm(comando);
            }
            Operador::NOT => {
                let comando = ComandoLLVM {
                    comando: Comandos::Not,
                    destino: Some(destino.clone()),
                    parametros: vec![
                        primeiro_parametro,
                    ],
                };
        
                self.gerar_comando_llvm(comando);
            }
            Operador::AND => {
                let comando = ComandoLLVM {
                    comando: Comandos::And,
                    destino: Some(destino.clone()),
                    parametros: vec![
                        primeiro_parametro,
                        segundo_parametro.unwrap(),
                    ],
                };
        
                self.gerar_comando_llvm(comando);
            }
            Operador::OR => {
                let comando = ComandoLLVM {
                    comando: Comandos::Or,
                    destino: Some(destino.clone()),
                    parametros: vec![
                        primeiro_parametro,
                        segundo_parametro.unwrap(),
                    ],
                };
        
                self.gerar_comando_llvm(comando);
            }
            Operador::ADD => {
                let comando = ComandoLLVM {
                    comando: Comandos::Somar,
                    destino: Some(destino.clone()),
                    parametros: vec![
                        primeiro_parametro,
                        segundo_parametro.unwrap(),
                    ],
                };
        
                self.gerar_comando_llvm(comando);
            }
            Operador::SUB => {
                let comando = ComandoLLVM {
                    comando: Comandos::Subtrair,
                    destino: Some(destino.clone()),
                    parametros: vec![
                        primeiro_parametro,
                        segundo_parametro.unwrap(),
                    ],
                };
        
                self.gerar_comando_llvm(comando);
            }
            Operador::MUL => {
                let comando = ComandoLLVM {
                    comando: Comandos::Multiplicar,
                    destino: Some(destino.clone()),
                    parametros: vec![
                        primeiro_parametro,
                        segundo_parametro.unwrap(),
                    ],
                };
        
                self.gerar_comando_llvm(comando);
            }
            Operador::DIV => {
                let comando = ComandoLLVM {
                    comando: Comandos::Dividir,
                    destino: Some(destino.clone()),
                    parametros: vec![
                        primeiro_parametro,
                        segundo_parametro.unwrap(),
                    ],
                };
        
                self.gerar_comando_llvm(comando);
            }
            Operador::DIVR => {
                let comando = ComandoLLVM {
                    comando: Comandos::RestoDaDivisao,
                    destino: Some(destino.clone()),
                    parametros: vec![
                        primeiro_parametro,
                        segundo_parametro.unwrap(),
                    ],
                };
        
                self.gerar_comando_llvm(comando);
            }
        }

        return destino;
    }

    pub fn gerar_chamada_de_bloco(&mut self, bloco: ParametroLLVM) {
        let comando = ComandoLLVM {
            comando: Comandos::ChamarBloco,
            destino: None,
            parametros: vec![
                bloco
            ],
        };
        self.gerar_comando_llvm(comando);
    }
}