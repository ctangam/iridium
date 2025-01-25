use nom::{bytes::complete::tag, character::complete::digit1, combinator::map_res, IResult};

use super::Token;

pub fn register_parsers(i: &[u8]) -> IResult<&[u8], Token> {
    let (i, _) = tag("$")(i)?;
    let (i, reg_num) = map_res(digit1, |digit_str: &[u8]| {
        std::str::from_utf8(digit_str).unwrap().parse::<u8>()
    })(i)?;
    Ok((i, Token::Register(reg_num)))
}

mod tests {
    use super::*;

    #[test]
    fn test_parse_register() {
        let result = register_parsers(b"$0");
        assert!(result.is_ok());
        let result = register_parsers(b"0");
        assert!(result.is_err());
        let result = register_parsers(b"$a");
        assert!(result.is_err());
    }
}
