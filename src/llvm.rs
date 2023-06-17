use crate::comum::*;

#[derive(Debug, Clone)]
pub struct LLVM {
    llvm_ir: String,
}

impl LLVM {
    pub fn inicializar() -> Self {
        Self {
            llvm_ir: String::new(),
        }
    }

    pub fn declarar_var_global(&mut self, nome: String, tipo: TiposDeDado) {
        self.llvm_ir = format!("{}@{} = global", self.llvm_ir, nome);

        match tipo {
            TiposDeDado::Char(_) => {
                self.llvm_ir = format!("{} i8 0, align 1\n", self.llvm_ir);
            }
            TiposDeDado::Int8(_) => {
                self.llvm_ir = format!("{} i8 0, align 1\n", self.llvm_ir);
            }
            TiposDeDado::Uint8(_) => {
                self.llvm_ir = format!("{} i8 0, align 1\n", self.llvm_ir);
            }
            TiposDeDado::Int16(_) => {
                self.llvm_ir = format!("{} i16 0, align 2\n", self.llvm_ir);
            }
            TiposDeDado::Uint16(_) => {
                self.llvm_ir = format!("{} i16 0, align 2\n", self.llvm_ir);
            }
            TiposDeDado::Int32(_) => {
                self.llvm_ir = format!("{} i32 0, align 4\n", self.llvm_ir);
            }
            TiposDeDado::Uint32(_) => {
                self.llvm_ir = format!("{} i32 0, align 4\n", self.llvm_ir);
            }
            TiposDeDado::Int64(_) => {
                self.llvm_ir = format!("{} i64 0, align 8\n", self.llvm_ir);
            }
            TiposDeDado::Uint64(_) => {
                self.llvm_ir = format!("{} i64 0, align 8\n", self.llvm_ir);
            }
            TiposDeDado::String(_) => {
                self.llvm_ir = format!("{} i8* null, align 8\n", self.llvm_ir);
            }
        }
    }

    pub fn obter_llvm_ir(&self) -> String {
        self.llvm_ir.to_string()
    }
}