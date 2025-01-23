use super::Token;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    opcode: Token,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}

pub fn instruction_one(i: &[u8]) -> nom::IResult<&[u8], AssemblerInstruction> {
    let (i, opcode) = super::opcode_parsers::opcode_load(i)?;
    let (i, _) = nom::character::complete::space1(i)?;
    let (i, register) = super::register_parsers::register_parsers(i)?;
    let (i, _) = nom::character::complete::space1(i)?;
    let (i, operand) = super::operand_parsers::integer_operand(i)?;
    let (i, _) = nom::character::complete::line_ending(i)?;

    Ok((
        i,
        AssemblerInstruction {
            opcode,
            operand1: Some(register),
            operand2: Some(operand),
            operand3: None,
        },
    ))
}


#[cfg(test)]
mod tests {
    use crate::instruction::Opcode;

    use super::*;

    #[test]
    fn test_parse_instruction_form_one() {
        let result = instruction_one(b"load $0 #100\n");
        assert_eq!(
            result,
            Ok((
                "".as_bytes(),
                AssemblerInstruction {
                    opcode: Token::Op(Opcode::LOAD),
                    operand1: Some(Token::Register(0)),
                    operand2: Some(Token::IntegerOperand(100)),
                    operand3: None
                }
            ))
        );
    }
}