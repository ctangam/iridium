use crate::instruction::Opcode;

pub mod opcode_parsers;
pub mod register_parsers;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Op(Opcode),
    Register(u8),

}