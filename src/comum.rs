#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum ParametroGenerico {
    VariavelGlobal(String),
    VariavelTemporaria(VariavelTemporaria),
    Imediato(String),
}














#[derive(Debug, Clone, PartialEq)]
pub struct VariavelTemporaria {
    pub nome: String,
    pub tipo: TipoDeDado,
}

#[derive(Debug, Clone, PartialEq)]


// ------------------------------------------------------
// Semântico
// ------------------------------------------------------




// ------------------------------------------------------
// LLVM
// ------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum TiposDeSimbolo {
    Imediato,
    VariavelGlobal,
    VariavelTemporaria,
    PonteiroDeString,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Simbolo {
    pub simbolo: String,
    pub tipo_do_simbulo: TiposDeSimbolo,
    pub tipo_de_dado: TipoDeDado,
    pub alinhamento: u8,
    pub tamanho: u8,
    pub inicializado: bool,
}

#[allow(dead_code)]
impl Simbolo {
    pub fn instanciar(
        simbolo: &str,
        tipo_do_simbulo: TiposDeSimbolo,
        tipo_de_dado: TipoDeDado,
        inicializado: bool,
    ) -> Self {
        

        Self {
            comando,
            alinhamento,
            tamanho,
            sinalizado,
            destino: String::from(destino), 
            parametros,
        }
    }
}