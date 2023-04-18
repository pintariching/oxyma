use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::value;
use nom::IResult;

pub fn boolean(input: &str) -> IResult<&str, bool> {
    let parse_true = value(true, tag("true"));
    let parse_false = value(false, tag("false"));

    alt((parse_true, parse_false))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boolean() {
        assert_eq!(boolean("true"), Ok(("", true)));
        assert_eq!(boolean("false"), Ok(("", false)));
    }
}
