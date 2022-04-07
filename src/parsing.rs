use nom::{bytes::complete::is_not, character::complete::char, sequence::delimited, IResult};

fn parens(s: &[u8]) -> IResult<&[u8], &[u8]> {
    delimited(char('('), is_not(")"), char(')'))(s)
}

#[cfg(test)]
mod parsing_tests {}
