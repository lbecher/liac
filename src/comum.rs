#[derive(Debug, Clone, PartialEq, PartialOrd)]
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


#[derive(Debug, Clone)]
pub enum TiposDeDado {
    Char(Option<char>),
    String(Option<String>),
    Int8(Option<i8>),
    Int16(Option<i16>),
    Int32(Option<i32>),
    Int64(Option<i64>),
    Uint8(Option<u8>),
    Uint16(Option<u16>),
    Uint32(Option<u32>),
    Uint64(Option<u64>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Acoes {
    Empilha(usize),
    Reduz(usize),
    VaiPara(usize),
    Aceita,
    Erro,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ElementosDaPilha {
    Tokens(Tokens),
    NaoTerminais(NaoTerminais),
    Estados(usize),
}