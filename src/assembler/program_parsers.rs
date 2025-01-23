use nom::{multi::many1, IResult};

use super::instruction_parsers::{instruction_one, AssemblerInstruction};

#[derive(Debug, PartialEq)]
pub struct Program {
    instructions: Vec<AssemblerInstruction>,
}

pub fn program(i: &[u8]) -> IResult<&[u8], Program> {
    let (i, instructions) = many1(instruction_one)(i)?;
    Ok((i, Program { instructions }))
}

mod tests{
    #[test]
    fn test_parse_program() {
        let result = super::program(b"load $0 #100\nload $1 #200\n");
        assert!(result.is_ok());
        let (rest, program) = result.unwrap();
        assert_eq!(rest, b"");
        assert_eq!(program.instructions.len(), 2);
    }
}