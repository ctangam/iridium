use crate::instruction::Opcode;

pub mod opcode_parsers;

#[derive(Debug)]
pub enum Token {
    Op{code: Opcode},
}