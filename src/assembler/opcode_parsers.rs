use nom::{branch::alt, bytes::complete::tag, character::complete::{self, alpha0}, combinator::map_res, IResult};

use crate::instruction::Opcode;

use super::Token;

pub fn opcode_load(i: &[u8]) -> IResult<&[u8], Token> {
    
}

mod tests {
    use crate::{assembler::{opcode_parsers::opcode_load, Token}, instruction::Opcode};

    #[test]
    fn test_opcode_load() {
        // First tests that the opcode is detected and parsed correctly
        let result = opcode_load(b"load");
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op{code: Opcode::LOAD});
        assert_eq!(rest, b"");

        // Tests that an invalid opcode isn't recognized
        let result = opcode_load(b"aold");
        assert_eq!(result.is_ok(), false);
    }
}