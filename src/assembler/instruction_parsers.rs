use nom::combinator::opt;

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
    let (i, _) = opt(nom::character::complete::line_ending)(i)?;

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

impl AssemblerInstruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut results = vec![];
        match self.opcode {
            Token::Op(code) => results.push(code as u8),
            _ => {
                println!("Non-opcode found in opcode field");
                std::process::exit(1);
            }
        };

        for operand in [&self.operand1, &self.operand2, &self.operand3] {
            match operand {
                Some(t) => AssemblerInstruction::extract_operand(t, &mut results),
                None => (),
            }
        }

        results
    }

    fn extract_operand(t: &Token, results: &mut Vec<u8>) {
        match t {
            Token::Register(reg_num) => results.push(*reg_num),
            Token::IntegerOperand(value) => {
                let converted = *value as u16;
                let byte1 = converted;
                let byte2 = converted >> 8;
                results.push(byte2 as u8);
                results.push(byte1 as u8);
            }
            _ => {
                println!("Opcode found in operand field");
                std::process::exit(1);
            }
        }
    }
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
