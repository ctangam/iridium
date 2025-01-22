use crate::instruction::Opcode;

pub mod opcode_parsers;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Op(Opcode),
    Register(u8),
    
}