use crate::instruction::Opcode;

pub mod instruction_parsers;
pub mod opcode_parsers;
pub mod operand_parsers;
pub mod program_parsers;
pub mod register_parsers;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op(Opcode),
    Register(u8),
    IntegerOperand(i32),
}
