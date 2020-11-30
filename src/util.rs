use capstone::prelude::*;
use capstone::Capstone;

use keystone::OptionValue;

pub enum SelectedColumn {
    Function,
    Hex,
    Disasm,
}

impl SelectedColumn {
    pub fn editable(&self) -> bool {
        match self {
            Self::Function => false,
            Self::Hex | Self::Disasm => true,
        }
    }
}

pub enum Mode {
    Viewing,
    Editing,
}

pub fn assemble(instr: String) -> Result<Vec<u8>, keystone::Error> {
    use keystone::{Arch, Keystone, OptionType};

    let engine = Keystone::new(
        Arch::X86,
        keystone::Mode::LITTLE_ENDIAN | keystone::Mode::MODE_64,
    )?;
    engine.option(OptionType::SYNTAX, OptionValue::SYNTAX_NASM)?;
    engine.asm(instr, 0x1000).map(|x| x.bytes)
}


pub fn disassemble(bytes: &[u8]) -> Vec<(Vec<u8>, String)> {
    let cs = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode64)
        .syntax(arch::x86::ArchSyntax::Intel)
        .detail(true)
        .build()
        .expect("failed to create capstone object");
    let insns = cs.disasm_all(bytes, 0x0).expect("disasm to work?");
    insns
        .iter()
        .map(|x| {
            (
                x.bytes().to_vec(),
                format!(
                    "{} {}",
                    x.mnemonic().unwrap_or(""),
                    x.op_str().unwrap_or("")
                ),
            )
        })
        .collect()
}

pub fn to_hexstring(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|x| format!("{:02x}", x))
        .collect::<Vec<String>>()
        .join(" ")
}
pub fn from_hexstring(str: String) -> Vec<u8> {
    str.chars()
        .filter(|x| *x != ' ')
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|x| u8::from_str_radix(&x.iter().collect::<String>(), 16).unwrap_or(0))
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assembles() {
        assert_eq!(vec![0x55], assemble("push rbp".to_string()).unwrap());
    }
}
