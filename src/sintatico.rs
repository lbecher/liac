use debug_print::debug_println;

use liac::*;
use crate::semantico::*;

#[derive(Debug, Clone)]
pub struct Sintatico {
    entrada: Vec<Tokens>,
    pilha: Vec<ElementosDaPilha>,
    simbolo_anterior: ElementosDaPilha,
    simbolo_atual: ElementosDaPilha,
    vai_para: bool,
    modo_panico: bool,
    quebras_de_linha: usize,
    semantico: Semantico,
}

impl Sintatico {
    pub fn inicializar(entrada: &Vec<Tokens>) -> Self {
        let mut pilha: Vec<ElementosDaPilha> = Vec::new();

        // adiciona estado 0 à pilha
        pilha.push(ElementosDaPilha::Estados(0));

        Sintatico {
            entrada: entrada.to_vec(),
            pilha: pilha,
            simbolo_anterior: ElementosDaPilha::Estados(0),
            simbolo_atual: ElementosDaPilha::Tokens(entrada[0].clone()),
            vai_para: false,
            modo_panico: false,
            quebras_de_linha: 0,
            semantico: Semantico::inicializar(),
        }
    }

    pub fn analisar(&mut self) {
        // vetor de tuplas que possui o não terminal e a quantidade de itens de cada produção da gramáticas
        let producoes = vec![
            (NaoTerminais::SL, 1 as usize),
            (NaoTerminais::S, 2 as usize),
            (NaoTerminais::S, 3 as usize),
            (NaoTerminais::A, 3 as usize),
            (NaoTerminais::B, 3 as usize),
            (NaoTerminais::C, 1 as usize),
            (NaoTerminais::C, 2 as usize),
            (NaoTerminais::D, 3 as usize),
            (NaoTerminais::E, 1 as usize),
            (NaoTerminais::E, 2 as usize),
            (NaoTerminais::F, 3 as usize),
            (NaoTerminais::G, 2 as usize),
            (NaoTerminais::G, 3 as usize),
            (NaoTerminais::H, 1 as usize),
            (NaoTerminais::H, 2 as usize),
            (NaoTerminais::I, 5 as usize),
            (NaoTerminais::I, 5 as usize),
            (NaoTerminais::I, 9 as usize),
            (NaoTerminais::I, 4 as usize),
            (NaoTerminais::I, 5 as usize),
            (NaoTerminais::I, 4 as usize),
            (NaoTerminais::I, 5 as usize),
            (NaoTerminais::I, 5 as usize),
            (NaoTerminais::I, 2 as usize),
            (NaoTerminais::J, 2 as usize),
            (NaoTerminais::J, 2 as usize),
            (NaoTerminais::J, 2 as usize),
            (NaoTerminais::J, 2 as usize),
            (NaoTerminais::J, 2 as usize),
            (NaoTerminais::K, 2 as usize),
            (NaoTerminais::K, 3 as usize),
            (NaoTerminais::K, 2 as usize),
            (NaoTerminais::K, 3 as usize),
            (NaoTerminais::K, 2 as usize),
            (NaoTerminais::K, 3 as usize),
            (NaoTerminais::K, 2 as usize),
            (NaoTerminais::K, 3 as usize),
            (NaoTerminais::K, 2 as usize),
            (NaoTerminais::K, 3 as usize),
            (NaoTerminais::L, 2 as usize),
            (NaoTerminais::L, 3 as usize),
            (NaoTerminais::M, 4 as usize),
            (NaoTerminais::M, 6 as usize),
            (NaoTerminais::N, 1 as usize),
            (NaoTerminais::N, 1 as usize),
            (NaoTerminais::N, 1 as usize),
            (NaoTerminais::O, 1 as usize),
            (NaoTerminais::P, 1 as usize),
        ];

        println!("---------------------------------------\nIniciando análise sintática...\n-------------\n");

        // coloca primeiro token no símbolo atual
        self.consome_quebra_de_linha();
        if self.entrada.len() == 0 {
            return;
        }
        self.simbolo_atual = ElementosDaPilha::Tokens(self.entrada[0].clone());

        loop {
            //debug_println!("Pilha: {:?}\nEntrada: {:?}", self.pilha, self.entrada);
            debug_println!("Pilha: {:?}", self.pilha);

            // obtem ação com base na tabela SLR
            if let Ok(acao) = self.obtem_acao()
            {
                debug_println!("Ação: {:?}\n", acao);

                if acao == Acoes::Aceita
                {
                    if self.modo_panico == true {
                        println!("-------------\nERRO SINTÁTICO: Token(s) inesperado(s) encontrado(s)!\n---------------------------------------\n");
                    } else {
                        self.semantico.gravar_codigo_llvm();
                        println!("-------------\nAnálise sintática terminou sem erros.\n---------------------------------------\n");
                    }
                    break;
                }
                else if let Acoes::Empilha(estado) = acao 
                {
                    // empilha símbolo atual na pilha
                    self.pilha.push(self.simbolo_atual.clone());

                    // empilha estado
                    self.pilha.push(ElementosDaPilha::Estados(estado));

                    if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = self.simbolo_atual.clone() {
                        self.semantico.empilha_bloco(Tokens::AbreBlocoINZ);
                    } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = self.simbolo_atual.clone() {
                        self.semantico.empilha_bloco(Tokens::AbreBlocoINZ);
                    }
                }
                else if let Acoes::Reduz(producao) = acao
                {
                    let mut desempilhados: Vec<ElementosDaPilha> = Vec::new();

                    // preserva não terminal
                    self.simbolo_anterior = self.simbolo_atual.clone();

                    // elimina elementos da pilha de acordo com o número de elementos da produção * 2
                    for _i in 0..(producoes[producao].1 + producoes[producao].1) {
                        desempilhados.push(self.pilha.pop().unwrap());
                    }

                    match self.semantico.tratar_desempilhamento(desempilhados, producao) {
                        Ok(_) => {}
                        Err(erro) => {
                            println!("-------------\n{}\n-------------\n", erro);
                        }
                    }

                    // coloca o não terminal obtido da produção no símbolo atual
                    self.consome_quebra_de_linha();
                    if self.entrada.len() == 0 {
                        break;
                    }
                    self.simbolo_atual = ElementosDaPilha::NaoTerminais(producoes[producao].0.clone());

                    // empilha o não terminal
                    self.pilha.push(ElementosDaPilha::NaoTerminais(producoes[producao].0.clone()));
                    
                    // ativa o modo vai para
                    self.vai_para = true;
                }
                else if let Acoes::VaiPara(estado) = acao
                {
                    // empilha novo estado
                    self.pilha.push(ElementosDaPilha::Estados(estado));

                    // restaura não terminal
                    self.consome_quebra_de_linha();
                    if self.entrada.len() == 0 {
                        break;
                    }
                    self.simbolo_atual = self.simbolo_anterior.clone();

                    // desativa modo vai para
                    self.vai_para = false;
                }
                else // Acoes::Erro
                {
                    if self.vai_para {
                        println!("-------------\nERRO: Token '{:?}' inesperado!\nLinha {}.\n-------------\n", self.simbolo_anterior, self.quebras_de_linha + 1);
                    } else {
                        println!("-------------\nERRO: Token '{:?}' inesperado!\nLinha {}.\n-------------\n", self.simbolo_atual, self.quebras_de_linha + 1);
                    }
                    
                    // ativa modo pânico
                    self.modo_panico = true;
                    
                    // remove token da entrada
                    self.entrada.remove(0);

                    // adiciona próximo token ao símbolo atual
                    self.consome_quebra_de_linha();
                    if self.entrada.len() == 0 {
                        break;
                    }
                    self.simbolo_atual = ElementosDaPilha::Tokens(self.entrada[0].clone());
                }

                // verifica se token já foi adicionado à pilha
                let index = self.pilha.len() - 2;
                if let ElementosDaPilha::Tokens(token) = self.pilha[index].clone() {
                    if ElementosDaPilha::Tokens(token) == self.simbolo_atual {
                        // remove token da entrada
                        self.entrada.remove(0);

                        // adiciona próximo token ao símbolo atual
                        self.consome_quebra_de_linha();
                        if self.entrada.len() == 0 {
                            break;
                        }
                        self.simbolo_atual = ElementosDaPilha::Tokens(self.entrada[0].clone());
                    }
                }
            }
            else
            {
                println!("-------------\nERRO INTERNO NO ANALISADOR SINTÁTICO!!!\n---------------------------------------\n");
                break;
            }
        }
    }

    fn consome_quebra_de_linha(&mut self) {
        if self.entrada.len() > 0 {
            while self.entrada[0] == Tokens::QuebraDeLinha {
                // incrementa linha
                self.quebras_de_linha += 1;

                // remove token da entrada
                self.entrada.remove(0);
            }
        }
    }

    fn obtem_acao(&mut self) -> Result<Acoes, ()> {
        // estado
        let estado: usize;
        let index_estado: usize;

        if self.vai_para == false {
            index_estado = self.pilha.len() - 1;
        } else {
            index_estado = self.pilha.len() - 2;
        }

        if let ElementosDaPilha::Estados(e) = self.pilha[index_estado] {
            estado = e;
        } else {
            println!("ERRO: O elemento na pilha não é um estado!");
            return Err(());
        }

        // símbolo
        let simbolo: ElementosDaPilha;

        if let ElementosDaPilha::Tokens(s) = self.simbolo_atual.to_owned() {
            simbolo = ElementosDaPilha::Tokens(s);
        } else if let ElementosDaPilha::NaoTerminais(s) = self.simbolo_atual.clone() {
            simbolo = ElementosDaPilha::NaoTerminais(s);
        } else {
            println!("ERRO: O elemento na pilha não é um símbolo de produção!");
            return Err(());
        }

        // tabela SLR
        match estado {
            0 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Empilha(102));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::S) = simbolo {
                    return Ok(Acoes::VaiPara(1));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::A) = simbolo {
                    return Ok(Acoes::VaiPara(2));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            1 => {
                if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Aceita);
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            2 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Empilha(99));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::B) = simbolo {
                    return Ok(Acoes::VaiPara(3));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            3 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Empilha(7));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::C) = simbolo {
                    return Ok(Acoes::VaiPara(4));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::D) = simbolo {
                    return Ok(Acoes::VaiPara(5));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            4 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            5 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(5));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(5));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(5));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(5));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Empilha(7));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(5));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(5));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(5));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(5));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(5));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(5));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(5));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(5));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(5));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(5));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(5));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(5));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(5));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(5));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(5));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(5));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(5));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(5));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(5));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(5));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(5));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(5));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(5));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(5));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(5));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::C) = simbolo {
                    return Ok(Acoes::VaiPara(6));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::D) = simbolo {
                    return Ok(Acoes::VaiPara(5));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            6 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(6));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(6));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(6));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(6));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(6));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(6));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(6));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(6));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(6));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(6));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(6));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(6));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(6));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(6));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(6));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(6));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(6));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(6));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(6));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(6));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(6));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(6));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(6));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(6));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(6));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(6));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(6));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(6));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(6));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(6));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            7 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Empilha(12));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Empilha(28));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Empilha(33));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Empilha(43));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Empilha(47));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Empilha(62));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Empilha(88));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Empilha(97));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::H) = simbolo {
                    return Ok(Acoes::VaiPara(8));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::I) = simbolo {
                    return Ok(Acoes::VaiPara(10));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            8 => {
                if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Empilha(9));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            9 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(7));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(7));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(7));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(7));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(7));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(7));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(7));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(7));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(7));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(7));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(7));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(7));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(7));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(7));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(7));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(7));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(7));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(7));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(7));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(7));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(7));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(7));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(7));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(7));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(7));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(7));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(7));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(7));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(7));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(7));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            10 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(13));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(13));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(13));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(13));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(13));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(13));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(13));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(13));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(13));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(13));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(13));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Empilha(12));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(13));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Empilha(28));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(13));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Empilha(33));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(13));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Empilha(43));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(13));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Empilha(47));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Empilha(62));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(13));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Empilha(88));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Empilha(97));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(13));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(13));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(13));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(13));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(13));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(13));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::H) = simbolo {
                    return Ok(Acoes::VaiPara(11));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::I) = simbolo {
                    return Ok(Acoes::VaiPara(10));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            11 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(14));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(14));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(14));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(14));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(14));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(14));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(14));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(14));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(14));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(14));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(14));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(14));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(14));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(14));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(14));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(14));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(14));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(14));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(14));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(14));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(14));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(14));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(14));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(14));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(14));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(14));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(14));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(14));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(14));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(14));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            12 => {
                if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Empilha(18));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Empilha(19));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Empilha(21));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::M) = simbolo {
                    return Ok(Acoes::VaiPara(20));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::N) = simbolo {
                    return Ok(Acoes::VaiPara(17));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::O) = simbolo {
                    return Ok(Acoes::VaiPara(13));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            13 => {
                if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Empilha(14));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            14 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Empilha(12));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Empilha(28));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Empilha(33));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Empilha(43));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Empilha(47));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Empilha(62));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Empilha(88));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Empilha(97));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::H) = simbolo {
                    return Ok(Acoes::VaiPara(15));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::I) = simbolo {
                    return Ok(Acoes::VaiPara(10));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            15 => {
                if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Empilha(16));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            16 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(15));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(15));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(15));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(15));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(15));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(15));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(15));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(15));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(15));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(15));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(15));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(15));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(15));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(15));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(15));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(15));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(15));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(15));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(15));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(15));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(15));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(15));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(15));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(15));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(15));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(15));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(15));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(15));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(15));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(15));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            17 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            18 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(43));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(43));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(43));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(43));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(43));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(43));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(43));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(43));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(43));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(43));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(43));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(43));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(43));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(43));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(43));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(43));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(43));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(43));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(43));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(43));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(43));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(43));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(43));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(43));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(43));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(43));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(43));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(43));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(43));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(43));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            19 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(44));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(44));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(44));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(44));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(44));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(44));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(44));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(44));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(44));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(44));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(44));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(44));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(44));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(44));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(44));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(44));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(44));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(44));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(44));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(44));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(44));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(44));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(44));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(44));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(44));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(44));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(44));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(44));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(44));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(44));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            20 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(45));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(45));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(45));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(45));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(45));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(45));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(45));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(45));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(45));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(45));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(45));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(45));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(45));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(45));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(45));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(45));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(45));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(45));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(45));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(45));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(45));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(45));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(45));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(45));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(45));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(45));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(45));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(45));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(45));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(45));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            21 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Empilha(22));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            22 => {
                if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Empilha(23));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            23 => {
                if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Empilha(25));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Empilha(24));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            24 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(41));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(41));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(41));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(41));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(41));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(41));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(41));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(41));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(41));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(41));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(41));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(41));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(41));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(41));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(41));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(41));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(41));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(41));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(41));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(41));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(41));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(41));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(41));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(41));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(41));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(41));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(41));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(41));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(41));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(41));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            25 => {
                if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Empilha(18));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Empilha(19));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Empilha(21));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::M) = simbolo {
                    return Ok(Acoes::VaiPara(20));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::N) = simbolo {
                    return Ok(Acoes::VaiPara(26));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            26 => {
                if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Empilha(27));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            27 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(42));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(42));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(42));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(42));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(42));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(42));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(42));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(42));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(42));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(42));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(42));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(42));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(42));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(42));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(42));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(42));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(42));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(42));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(42));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(42));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(42));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(42));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(42));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(42));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(42));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(42));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(42));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(42));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(42));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(42));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            28 => {
                if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Empilha(18));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Empilha(19));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Empilha(21));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::M) = simbolo {
                    return Ok(Acoes::VaiPara(20));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::N) = simbolo {
                    return Ok(Acoes::VaiPara(17));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::O) = simbolo {
                    return Ok(Acoes::VaiPara(29));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            29 => {
                if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Empilha(30));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            30 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Empilha(12));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Empilha(28));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Empilha(33));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Empilha(43));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Empilha(47));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Empilha(62));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Empilha(88));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Empilha(97));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::H) = simbolo {
                    return Ok(Acoes::VaiPara(31));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::I) = simbolo {
                    return Ok(Acoes::VaiPara(10));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            31 => {
                if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Empilha(32));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            32 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(16));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(16));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(16));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(16));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(16));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(16));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(16));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(16));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(16));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(16));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(16));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(16));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(16));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(16));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(16));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(16));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(16));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(16));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(16));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(16));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(16));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(16));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(16));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(16));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(16));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(16));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(16));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(16));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(16));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(16));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            33 => {
                if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Empilha(34));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            34 => {
                if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Empilha(35));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            35 => {
                if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Empilha(18));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Empilha(19));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Empilha(21));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::M) = simbolo {
                    return Ok(Acoes::VaiPara(20));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::N) = simbolo {
                    return Ok(Acoes::VaiPara(17));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::O) = simbolo {
                    return Ok(Acoes::VaiPara(36));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            36 => {
                if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Empilha(37));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            37 => {
                if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Empilha(18));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Empilha(19));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Empilha(21));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::M) = simbolo {
                    return Ok(Acoes::VaiPara(20));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::N) = simbolo {
                    return Ok(Acoes::VaiPara(42));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::P) = simbolo {
                    return Ok(Acoes::VaiPara(38));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            38 => {
                if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Empilha(39));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            39 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Empilha(12));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Empilha(28));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Empilha(33));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Empilha(43));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Empilha(47));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Empilha(62));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Empilha(88));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Empilha(97));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::H) = simbolo {
                    return Ok(Acoes::VaiPara(40));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::I) = simbolo {
                    return Ok(Acoes::VaiPara(10));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            40 => {
                if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Empilha(41));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            41 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(17));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(17));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(17));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(17));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(17));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(17));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(17));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(17));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(17));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(17));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(17));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(17));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(17));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(17));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(17));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(17));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(17));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(17));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(17));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(17));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(17));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(17));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(17));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(17));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(17));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(17));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(17));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(17));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(17));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(17));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            42 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(46));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            43 => {
                if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Empilha(44));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            44 => {
                if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Empilha(45));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            45 => {
                if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Empilha(46));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            46 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(18));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(18));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(18));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(18));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(18));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(18));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(18));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(18));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(18));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(18));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(18));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(18));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(18));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(18));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(18));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(18));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(18));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(18));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(18));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(18));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(18));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(18));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(18));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(18));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(18));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(18));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(18));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(18));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(18));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(18));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            47 => {
                if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Empilha(48));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            48 => {
                if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Empilha(49));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            49 => {
                if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Empilha(50));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            50 => {
                if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Empilha(54));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Empilha(58));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Empilha(52));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Empilha(56));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Empilha(21));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::J) = simbolo {
                    return Ok(Acoes::VaiPara(51));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::M) = simbolo {
                    return Ok(Acoes::VaiPara(60));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            51 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(19));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(19));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(19));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(19));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(19));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(19));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(19));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(19));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(19));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(19));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(19));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(19));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(19));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(19));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(19));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(19));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(19));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(19));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(19));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(19));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(19));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(19));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(19));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(19));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(19));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(19));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(19));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(19));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(19));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(19));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            52 => {
                if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Empilha(53));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            53 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(24));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(24));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(24));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(24));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(24));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(24));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(24));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(24));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(24));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(24));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(24));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(24));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(24));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(24));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(24));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(24));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(24));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(24));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(24));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(24));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(24));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(24));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(24));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(24));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(24));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(24));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(24));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(24));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(24));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(24));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            54 => {
                if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Empilha(55));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            55 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(25));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(25));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(25));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(25));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(25));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(25));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(25));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(25));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(25));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(25));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(25));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(25));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(25));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(25));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(25));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(25));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(25));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(25));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(25));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(25));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(25));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(25));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(25));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(25));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(25));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(25));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(25));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(25));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(25));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(25));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            56 => {
                if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Empilha(57));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            57 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(26));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(26));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(26));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(26));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(26));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(26));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(26));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(26));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(26));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(26));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(26));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(26));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(26));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(26));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(26));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(26));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(26));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(26));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(26));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(26));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(26));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(26));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(26));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(26));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(26));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(26));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(26));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(26));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(26));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(26));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            58 => {
                if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Empilha(59));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            59 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(27));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(27));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(27));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(27));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(27));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(27));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(27));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(27));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(27));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(27));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(27));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(27));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(27));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(27));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(27));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(27));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(27));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(27));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(27));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(27));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(27));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(27));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(27));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(27));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(27));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(27));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(27));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(27));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(27));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(27));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            60 => {
                if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Empilha(61));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            61 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            62 => {
                if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Empilha(63));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            63 => {
                if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Empilha(64));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            64 => {
                if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Empilha(65));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Empilha(66));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            65 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(20));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(20));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(20));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(20));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(20));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(20));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(20));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(20));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(20));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(20));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(20));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(20));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(20));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(20));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(20));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(20));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(20));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(20));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(20));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(20));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(20));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(20));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(20));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(20));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(20));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(20));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(20));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(20));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(20));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(20));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            66 => {
                if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Empilha(72));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Empilha(80));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Empilha(68));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Empilha(76));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Empilha(21));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::K) = simbolo {
                    return Ok(Acoes::VaiPara(67));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::M) = simbolo {
                    return Ok(Acoes::VaiPara(84));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            67 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(21));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(21));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(21));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(21));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(21));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(21));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(21));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(21));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(21));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(21));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(21));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(21));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(21));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(21));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(21));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(21));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(21));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(21));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(21));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(21));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(21));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(21));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(21));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(21));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(21));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(21));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(21));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(21));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(21));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(21));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            68 => {
                if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Empilha(69));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Empilha(70));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            69 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(29));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(29));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(29));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(29));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(29));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(29));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(29));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(29));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(29));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(29));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(29));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(29));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(29));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(29));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(29));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(29));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(29));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(29));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(29));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(29));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(29));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(29));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(29));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(29));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(29));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(29));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(29));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(29));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(29));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(29));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            70 => {
                if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Empilha(72));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Empilha(80));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Empilha(68));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Empilha(76));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Empilha(21));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::K) = simbolo {
                    return Ok(Acoes::VaiPara(71));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::M) = simbolo {
                    return Ok(Acoes::VaiPara(84));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            71 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(30));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(30));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(30));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(30));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(30));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(30));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(30));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(30));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(30));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(30));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(30));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(30));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(30));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(30));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(30));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(30));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(30));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(30));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(30));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(30));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(30));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(30));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(30));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(30));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(30));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(30));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(30));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(30));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(30));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(30));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            72 => {
                if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Empilha(73));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Empilha(74));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            73 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(31));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(31));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(31));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(31));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(31));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(31));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(31));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(31));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(31));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(31));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(31));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(31));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(31));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(31));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(31));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(31));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(31));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(31));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(31));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(31));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(31));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(31));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(31));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(31));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(31));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(31));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(31));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(31));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(31));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(31));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            74 => {
                if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Empilha(72));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Empilha(80));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Empilha(68));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Empilha(76));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Empilha(21));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::K) = simbolo {
                    return Ok(Acoes::VaiPara(75));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::M) = simbolo {
                    return Ok(Acoes::VaiPara(84));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            75 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(32));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(32));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(32));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(32));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(32));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(32));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(32));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(32));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(32));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(32));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(32));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(32));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(32));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(32));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(32));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(32));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(32));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(32));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(32));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(32));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(32));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(32));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(32));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(32));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(32));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(32));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(32));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(32));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(32));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(32));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            76 => {
                if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Empilha(77));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Empilha(78));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            77 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(33));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(33));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(33));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(33));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(33));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(33));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(33));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(33));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(33));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(33));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(33));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(33));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(33));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(33));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(33));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(33));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(33));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(33));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(33));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(33));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(33));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(33));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(33));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(33));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(33));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(33));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(33));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(33));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(33));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(33));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            78 => {
                if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Empilha(72));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Empilha(80));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Empilha(68));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Empilha(76));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Empilha(21));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::K) = simbolo {
                    return Ok(Acoes::VaiPara(79));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::M) = simbolo {
                    return Ok(Acoes::VaiPara(84));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            79 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(34));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(34));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(34));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(34));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(34));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(34));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(34));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(34));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(34));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(34));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(34));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(34));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(34));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(34));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(34));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(34));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(34));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(34));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(34));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(34));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(34));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(34));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(34));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(34));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(34));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(34));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(34));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(34));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(34));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(34));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            80 => {
                if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Empilha(81));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Empilha(82));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            81 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(35));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(35));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(35));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(35));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(35));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(35));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(35));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(35));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(35));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(35));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(35));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(35));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(35));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(35));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(35));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(35));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(35));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(35));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(35));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(35));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(35));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(35));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(35));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(35));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(35));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(35));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(35));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(35));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(35));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(35));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            82 => {
                if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Empilha(72));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Empilha(80));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Empilha(68));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Empilha(76));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Empilha(21));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::K) = simbolo {
                    return Ok(Acoes::VaiPara(83));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::M) = simbolo {
                    return Ok(Acoes::VaiPara(84));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            83 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(36));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(36));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(36));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(36));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(36));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(36));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(36));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(36));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(36));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(36));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(36));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(36));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(36));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(36));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(36));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(36));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(36));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(36));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(36));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(36));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(36));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(36));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(36));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(36));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(36));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(36));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(36));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(36));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(36));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(36));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            84 => {
                if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Empilha(85));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Empilha(86));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            85 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(28));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            86 => {
                if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Empilha(72));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Empilha(80));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Empilha(68));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Empilha(76));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Empilha(21));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::K) = simbolo {
                    return Ok(Acoes::VaiPara(87));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::M) = simbolo {
                    return Ok(Acoes::VaiPara(84));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            87 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(38));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(38));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(38));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(38));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(38));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(38));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(38));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(38));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(38));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(38));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(38));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(38));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(38));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(38));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(38));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(38));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(38));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(38));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(38));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(38));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(38));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(38));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(38));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(38));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(38));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(38));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(38));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(38));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(38));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(38));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            88 => {
                if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Empilha(89));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            89 => {
                if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Empilha(90));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            90 => {
                if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Empilha(91));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            91 => {
                if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Empilha(93));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::L) = simbolo {
                    return Ok(Acoes::VaiPara(92));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            92 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(22));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(22));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(22));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(22));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(22));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(22));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(22));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(22));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(22));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(22));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(22));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(22));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(22));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(22));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(22));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(22));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(22));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(22));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(22));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(22));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(22));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(22));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(22));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(22));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(22));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(22));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(22));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(22));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(22));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(22));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            93 => {
                if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Empilha(94));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Empilha(95));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            94 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(39));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(39));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(39));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(39));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(39));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(39));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(39));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(39));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(39));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(39));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(39));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(39));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(39));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(39));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(39));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(39));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(39));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(39));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(39));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(39));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(39));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(39));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(39));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(39));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(39));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(39));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(39));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(39));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(39));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(39));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            95 => {
                if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Empilha(93));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::L) = simbolo {
                    return Ok(Acoes::VaiPara(96));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            96 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(40));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(40));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(40));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(40));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(40));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(40));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(40));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(40));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(40));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(40));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(40));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(40));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(40));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(40));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(40));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(40));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(40));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(40));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(40));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(40));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(40));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(40));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(40));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(40));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(40));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(40));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(40));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(40));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(40));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(40));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            97 => {
                if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Empilha(98));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            98 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(23));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(23));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(23));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(23));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(23));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(23));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(23));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(23));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(23));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(23));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(23));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(23));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(23));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(23));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(23));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(23));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(23));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(23));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(23));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(23));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(23));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(23));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(23));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(23));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(23));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(23));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(23));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(23));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(23));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(23));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            99 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Empilha(12));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Empilha(28));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Empilha(33));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Empilha(43));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Empilha(47));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Empilha(62));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Empilha(88));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Empilha(97));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::H) = simbolo {
                    return Ok(Acoes::VaiPara(100));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::I) = simbolo {
                    return Ok(Acoes::VaiPara(10));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            100 => {
                if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Empilha(101));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            101 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            102 => {
                if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Empilha(107));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::E) = simbolo {
                    return Ok(Acoes::VaiPara(103));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::F) = simbolo {
                    return Ok(Acoes::VaiPara(105));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            103 => {
                if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Empilha(104));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            104 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            105 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(8));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(8));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(8));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(8));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(8));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(8));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Empilha(107));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(8));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(8));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(8));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(8));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(8));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(8));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(8));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(8));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(8));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(8));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(8));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(8));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(8));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(8));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(8));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(8));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(8));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(8));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(8));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(8));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(8));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(8));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(8));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::E) = simbolo {
                    return Ok(Acoes::VaiPara(106));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::F) = simbolo {
                    return Ok(Acoes::VaiPara(105));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            106 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(9));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(9));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(9));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(9));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(9));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(9));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(9));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(9));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(9));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(9));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(9));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(9));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(9));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(9));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(9));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(9));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(9));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(9));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(9));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(9));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(9));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(9));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(9));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(9));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(9));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(9));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(9));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(9));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(9));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(9));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            107 => {
                if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Empilha(108));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            108 => {
                if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Empilha(110));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::G) = simbolo {
                    return Ok(Acoes::VaiPara(109));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            109 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(10));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(10));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(10));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(10));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(10));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(10));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(10));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(10));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(10));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(10));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(10));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(10));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(10));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(10));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(10));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(10));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(10));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(10));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(10));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(10));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(10));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(10));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(10));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(10));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(10));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(10));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(10));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(10));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(10));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(10));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            110 => {
                if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Empilha(111));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Empilha(112));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            111 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(11));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(11));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(11));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(11));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(11));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(11));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(11));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(11));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(11));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(11));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(11));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(11));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(11));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(11));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(11));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(11));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(11));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(11));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(11));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(11));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(11));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(11));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(11));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(11));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(11));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(11));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(11));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(11));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(11));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(11));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            112 => {
                if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Empilha(110));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::G) = simbolo {
                    return Ok(Acoes::VaiPara(113));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            113 => {
                if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(12));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDATA) = simbolo {
                    return Ok(Acoes::Reduz(12));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(12));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoMAIN) = simbolo {
                    return Ok(Acoes::Reduz(12));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(12));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoDeCodigo(_)) = simbolo {
                    return Ok(Acoes::Reduz(12));
                } else if let ElementosDaPilha::Tokens(Tokens::TipoDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(12));
                } else if let ElementosDaPilha::Tokens(Tokens::DoisPontos) = simbolo {
                    return Ok(Acoes::Reduz(12));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeVariavel(_)) = simbolo {
                    return Ok(Acoes::Reduz(12));
                } else if let ElementosDaPilha::Tokens(Tokens::PontoEVirgula) = simbolo {
                    return Ok(Acoes::Reduz(12));
                } else if let ElementosDaPilha::Tokens(Tokens::Virgula) = simbolo {
                    return Ok(Acoes::Reduz(12));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(12));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoINZ) = simbolo {
                    return Ok(Acoes::Reduz(12));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(12));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoWNZ) = simbolo {
                    return Ok(Acoes::Reduz(12));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(12));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaBlocoRUI) = simbolo {
                    return Ok(Acoes::Reduz(12));
                } else if let ElementosDaPilha::Tokens(Tokens::Bloc) = simbolo {
                    return Ok(Acoes::Reduz(12));
                } else if let ElementosDaPilha::Tokens(Tokens::IdDeBloco(_)) = simbolo {
                    return Ok(Acoes::Reduz(12));
                } else if let ElementosDaPilha::Tokens(Tokens::Set) = simbolo {
                    return Ok(Acoes::Reduz(12));
                } else if let ElementosDaPilha::Tokens(Tokens::Print) = simbolo {
                    return Ok(Acoes::Reduz(12));
                } else if let ElementosDaPilha::Tokens(Tokens::String(_)) = simbolo {
                    return Ok(Acoes::Reduz(12));
                } else if let ElementosDaPilha::Tokens(Tokens::Scan) = simbolo {
                    return Ok(Acoes::Reduz(12));
                } else if let ElementosDaPilha::Tokens(Tokens::Return) = simbolo {
                    return Ok(Acoes::Reduz(12));
                } else if let ElementosDaPilha::Tokens(Tokens::Numero(_)) = simbolo {
                    return Ok(Acoes::Reduz(12));
                } else if let ElementosDaPilha::Tokens(Tokens::Caractere(_)) = simbolo {
                    return Ok(Acoes::Reduz(12));
                } else if let ElementosDaPilha::Tokens(Tokens::Operador(_)) = simbolo {
                    return Ok(Acoes::Reduz(12));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreParenteses) = simbolo {
                    return Ok(Acoes::Reduz(12));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaParenteses) = simbolo {
                    return Ok(Acoes::Reduz(12));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(12));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            _ => {
                return Err(());
            },
        }
    }
}