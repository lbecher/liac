
// ---------------------------------
// Léxico 
// ---------------------------------

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


// ---------------------------------
// Sintático 
// ---------------------------------

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
    M,
    N,
    O,
    P,
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


// ---------------------------------
// Semântico 
// ---------------------------------

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TipoDeDado {
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
    Undefined,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariavelGlobal {
    pub tipo: TipoDeDado,
    pub inicializada: bool,
}


// ---------------------------------
// LLVM 
// ---------------------------------

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
    A,
    B,
    E,
    AE,
    BE,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Comandos {
    ChamarBloco,
    DeclararString,
    DeclararVariavel,
    GravarVariavel,
    LerVariavel,
    Print,
    Scan,
    Dividir,
    Multiplicar,
    Somar,
    Subtrair,
    RestoDaDivisao,
    Igual,
    Maior,
    Menor,
    MaiorIgual,
    MenorIgual,
    ConverterBooleano,
    And,
    Not,
    Or,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TipoDeParametroLLVM {
    Numero,
    VariavelGlobal,
    VariavelTemporaria,
    String,
    Caractere,
    Bloco,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParametroLLVM {
    pub parametro: String,
    pub tipo_de_parametro: TipoDeParametroLLVM,
    pub tipo_de_dado: TipoDeDado,
    pub tamanho: u8,
    pub alinhamento: u8,
    pub tamanho_do_array: Option<usize>,
    pub sinalizado: bool,
}

impl ParametroLLVM {
    pub fn instanciar(
        parametro: &str,
        tipo_de_parametro: TipoDeParametroLLVM,
        tipo_de_dado: TipoDeDado,
    ) -> Self {
        let parametro = String::from(parametro);

        let (tamanho, alinhamento, sinalizado) = match tipo_de_dado.clone() {
            TipoDeDado::Char => (8, 1, false),
            TipoDeDado::String => (8, 8, false),
            TipoDeDado::Int8 => (8, 1, true),
            TipoDeDado::Int16 => (16, 2, true),
            TipoDeDado::Int32 => (32, 4, true),
            TipoDeDado::Int64 => (64, 8, true),
            TipoDeDado::Uint8 => (8, 1, false),
            TipoDeDado::Uint16 => (16, 2, false),
            TipoDeDado::Uint32 => (32, 4, false),
            TipoDeDado::Uint64 => (64, 8, false),
            TipoDeDado::Undefined => (64, 8, false),
        };

        let mut tamanho_array: usize = 0;

        Self {
            parametro: match tipo_de_parametro.clone() {
                TipoDeParametroLLVM::Numero => parametro.to_string(),
                TipoDeParametroLLVM::String => {
                    let mut string = parametro.to_string();
                    string += "\0";
                    tamanho_array = string.len();
                    string = string.replace("\\", "\\5C");
                    string = string.replace("\0", "\\00");
                    string = string.replace("\n", "\\0A");
                    string = string.replace("\t", "\\09");
                    string = string.replace("\r", "\\0D");
                    string = string.replace("\"", "\\22");
                    string = string.replace("\'", "\\27");
                    format!("c\"{}\"", string)
                },
                TipoDeParametroLLVM::VariavelGlobal => format!("@{}", parametro),
                TipoDeParametroLLVM::VariavelTemporaria => format!("%{}", parametro),
                TipoDeParametroLLVM::Caractere => format!("'{}'", parametro),
                TipoDeParametroLLVM::Bloco => format!("@bloc_{}()", parametro),
            },
            tipo_de_parametro: tipo_de_parametro.clone(),
            tipo_de_dado,
            tamanho,
            alinhamento,
            tamanho_do_array: if tipo_de_parametro == TipoDeParametroLLVM::String {
                Some(tamanho_array)
            } else {
                None
            },
            sinalizado,
        }
    }

    pub fn setar_tamanho_do_array(&mut self, tamanho: Option<usize>) {
        self.tamanho_do_array = tamanho;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComandoLLVM {
    pub comando: Comandos,
    pub destino: Option<ParametroLLVM>,
    pub parametros: Vec<ParametroLLVM>,
}