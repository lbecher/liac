use regex::Regex;

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
    Return,
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

pub fn obter_maximo_minimo(texto: &str) -> Option<(usize, usize)> {
    // Define a expressão regular para encontrar padrões como %<numero> ou <numero>:
    let re = Regex::new(r"%(\d+)|(\d+):").unwrap();

    // Variáveis para armazenar o máximo e o mínimo encontrados:
    let mut maximo: Option<usize> = None;
    let mut minimo: Option<usize> = None;

    // Itera sobre as correspondências encontradas na string:
    for caps in re.captures_iter(texto) {
        if let Some(numero) = caps.get(1) {
            // Se for do tipo %<numero>
            let numero_atual = numero.as_str().parse::<usize>().unwrap();
            maximo = Some(maximo.unwrap_or(numero_atual).max(numero_atual));
            minimo = Some(minimo.unwrap_or(numero_atual).min(numero_atual));
        } else if let Some(numero) = caps.get(2) {
            // Se for do tipo <numero>:
            let numero_atual = numero.as_str().parse::<usize>().unwrap();
            maximo = Some(maximo.unwrap_or(numero_atual).max(numero_atual));
            minimo = Some(minimo.unwrap_or(numero_atual).min(numero_atual));
        }
    }

    // Retorna o resultado como uma tupla opcional
    maximo.and_then(|max| minimo.map(|min| (min, max)))
}

pub fn substituir_com_incremento(texto: &str, incremento: usize) -> String {
    // Define a expressão regular para encontrar padrões como %<numero> ou <numero>:
    let re = Regex::new(r"%(\d+)|(\d+):").unwrap();

    // Substitui os padrões na string usando uma closure:
    let resultado = re.replace_all(texto, |caps: &regex::Captures| {
        if let Some(numero) = caps.get(1) {
            // Se for do tipo %<numero>, faz a substituição
            let numero_antigo = numero.as_str().parse::<usize>().unwrap();
            let novo_numero = numero_antigo.wrapping_add(incremento); // Usamos wrapping_add para evitar estouro numérico.
            format!("%{}", novo_numero)
        } else if let Some(numero) = caps.get(2) {
            // Se for do tipo <numero>:
            let numero_antigo = numero.as_str().parse::<usize>().unwrap();
            let novo_numero = numero_antigo.wrapping_add(incremento); // Usamos wrapping_add para evitar estouro numérico.
            format!("{}:", novo_numero)
        } else {
            // Se não for um padrão conhecido, retorna o próprio texto.
            caps.get(0).unwrap().as_str().to_string()
        }
    });

    resultado.to_string()
}

pub fn substituir_com_decremento(texto: &str, decremento: usize) -> String {
    // Define a expressão regular para encontrar padrões como %<numero> ou <numero>:
    let re = Regex::new(r"%(\d+)|(\d+):").unwrap();

    // Substitui os padrões na string usando uma closure:
    let resultado = re.replace_all(texto, |caps: &regex::Captures| {
        if let Some(numero) = caps.get(1) {
            // Se for do tipo %<numero>, faz a substituição
            let numero_antigo = numero.as_str().parse::<usize>().unwrap();
            let novo_numero = numero_antigo.wrapping_sub(decremento); // Usamos wrapping_sub para evitar estouro numérico.
            format!("%{}", novo_numero)
        } else if let Some(numero) = caps.get(2) {
            // Se for do tipo <numero>:
            let numero_antigo = numero.as_str().parse::<usize>().unwrap();
            let novo_numero = numero_antigo.wrapping_sub(decremento); // Usamos wrapping_sub para evitar estouro numérico.
            format!("{}:", novo_numero)
        } else {
            // Se não for um padrão conhecido, retorna o próprio texto.
            caps.get(0).unwrap().as_str().to_string()
        }
    });

    resultado.to_string()
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
    Diferente,
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