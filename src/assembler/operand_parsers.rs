use nom::{bytes::complete::tag, character::complete::digit1, combinator::map_res, IResult};

use super::Token;

pub fn integer_operand(i: &[u8]) -> IResult<&[u8], Token> {
    let (i, _) = tag("#")(i)?;
    let (i, int_operand) = map_res(digit1, |digit_str: &[u8]| {
        std::str::from_utf8(digit_str).unwrap().parse::<i32>()
    })(i)?;
    Ok((i, Token::IntegerOperand(int_operand)))
}

mod tests {
    use super::*;

    #[test]
    fn test_parse_integer_operand() {
        let result = integer_operand(b"#10");
        assert!(result.is_ok());
        let result = integer_operand(b"10");
        assert!(result.is_err());
        let result = integer_operand(b"#a");
        assert!(result.is_err());
    }
}
