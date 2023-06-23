#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum ParametroGenerico {
    VariavelGlobal(String),
    VariavelTemporaria(VariavelTemporaria),
    Imediato(String),
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Comando {
    DeclararString,
    DeclararVarGlobal,
    GravarVarGlobal,
    LerVarGlobal,
    Print,
    Scan,
    Dividir,
    Multiplicar,
    Somar,
    Subtrair,
    Resto,
    Igual,
    Diferente,
    Maior,
    Menor,
    MaiorIgual,
    MenorIgual,
    Converter,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Tokens {
    Fim,
    QuebraDeLinha,
    Virgula,
    DoisPontos,
    PontoEVirgula,
    AbreParenteses,
    FechaParenteses,
    Set,
    Print,
    Scan,
    Bloc,
    AbreBlocoINZ,
    FechaBlocoINZ,
    AbreBlocoWNZ,
    FechaBlocoWNZ,
    AbreBlocoRUI,
    FechaBlocoRUI,
    AbreBlocoDATA,
    FechaBlocoDATA,
    AbreBlocoMAIN,
    FechaBlocoMAIN,
    AbreBlocoDeCodigo(String),
    FechaBlocoDeCodigo(String),
    Operador(String),
    TipoDeVariavel(String),
    IdDeVariavel(String),
    IdDeBloco(String),
    Caractere(String),
    Numero(String),
    String(String),
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TiposDeDado {
    Char,
    String,
    Int8,
    Int16,
    Int32,
    Int64,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Operador {
    OR,
    AND,
    NOT,
    ADD,
    SUB,
    MUL,
    DIV,
    DIVR,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum NaoTerminais {
    SL,
    S,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Acoes {
    Empilha(usize),
    Reduz(usize),
    VaiPara(usize),
    Aceita,
    Erro,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum ElementosDaPilha {
    Tokens(Tokens),
    NaoTerminais(NaoTerminais),
    Estados(usize),
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariavelGlobal {
    pub tipo: TiposDeDado,
    pub inicializada: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariavelTemporaria {
    pub nome: String,
    pub tipo: TiposDeDado,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OpcoesDoComando {
    pub comando: Comando,
    pub alinhamento: u8,
    pub tamanho: u8,
    pub sinalizado: bool,
    pub destino: String,
    pub parametros: [String; 2],
}

#[allow(dead_code)]
impl OpcoesDoComando {
    pub fn instanciar(
        comando: Comando,
        tipo_de_dado: TiposDeDado,
        destino: &str,
        parametros: [&str; 2],
    ) -> Self {
        let (tamanho, alinhamento, sinalizado) = match tipo_de_dado {
            TiposDeDado::Char => (8, 1, false),
            TiposDeDado::String => (8, 8, false),
            TiposDeDado::Int8 => (8, 1, true),
            TiposDeDado::Int16 => (16, 2, true),
            TiposDeDado::Int32 => (32, 4, true),
            TiposDeDado::Int64 => (64, 8, true),
            TiposDeDado::Uint8 => (8, 1, false),
            TiposDeDado::Uint16 => (16, 2, false),
            TiposDeDado::Uint32 => (32, 4, false),
            TiposDeDado::Uint64 => (64, 8, false),
        };

        Self {
            comando,
            alinhamento,
            tamanho,
            sinalizado,
            destino: String::from(destino), 
            parametros: [String::from(parametros[0]), String::from(parametros[1])],
        }
    }
}