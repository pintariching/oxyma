use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::take;
use nom::character::complete::char;
use nom::combinator::map;
use nom::combinator::map_opt;
use nom::combinator::map_res;
use nom::combinator::value;
use nom::combinator::verify;
use nom::multi::fold_many0;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::IResult;

fn character_code(input: &str) -> IResult<&str, char> {
    let take_3 = take(3usize);
    let parse_u32 = map_res(take_3, |oct| u32::from_str_radix(oct, 8));

    map_opt(parse_u32, char::from_u32)(input)
}

fn parse_escaped_char(input: &str) -> IResult<&str, char> {
    preceded(
        char('\\'),
        alt((
            value('\n', char('n')),
            value('\r', char('r')),
            value('\t', char('t')),
            value('b', char('b')),
            value('f', char('f')),
            value('(', char('(')),
            value(')', char(')')),
            value('\\', char('\\')),
            character_code,
        )),
    )(input)
}

#[derive(Debug, PartialEq)]
enum StringFragment<'a> {
    Literal(&'a str),
    EscapedChar(char),
}

fn parse_literal(input: &str) -> IResult<&str, &str> {
    let not_quote_slash_par = is_not("\"\\()");

    verify(not_quote_slash_par, |s: &str| !s.is_empty())(input)
}

fn parse_fragment(input: &str) -> IResult<&str, StringFragment> {
    alt((
        map(parse_literal, StringFragment::Literal),
        map(parse_escaped_char, StringFragment::EscapedChar),
    ))(input)
}

pub fn string(input: &str) -> IResult<&str, String> {
    let build_string = fold_many0(parse_fragment, String::new, |mut string, fragment| {
        match fragment {
            StringFragment::Literal(l) => string.push_str(l),
            StringFragment::EscapedChar(c) => string.push(c),
        }
        string
    });

    delimited(char('('), build_string, char(')'))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::Finish;

    #[test]
    fn test_string() {
        assert_eq!(string("(abc)"), Ok(("", "abc".to_string())));
        assert_eq!(string("(abc \n def)"), Ok(("", "abc \n def".to_string())));
        assert_eq!(string(r"(abc \101)"), Ok(("", "abc A".to_string())));
        assert_eq!(string(r"(abc \1013)"), Ok(("", "abc A3".to_string())));
    }

    #[test]
    fn test_character_code() {
        assert_eq!(character_code("101"), Ok(("", 'A')));
        assert_eq!(character_code("043"), Ok(("", '#')));
        assert_eq!(character_code("04342"), Ok(("42", '#')));

        assert!(character_code("10a").finish().is_err());
    }

    #[test]
    fn test_parse_escaped_char() {
        assert_eq!(parse_escaped_char(r"\101"), Ok(("", 'A')));
        assert_eq!(parse_escaped_char(r"\043"), Ok(("", '#')));
        assert_eq!(parse_escaped_char(r"\n"), Ok(("", '\n')));
        assert_eq!(parse_escaped_char(r"\\"), Ok(("", '\\')));
        assert_eq!(parse_escaped_char(r"\b"), Ok(("", 'b')));
        assert_eq!(parse_escaped_char(r"\t"), Ok(("", '\t')));
        assert_eq!(parse_escaped_char(r"\("), Ok(("", '(')));
    }

    #[test]
    fn test_parse_literal() {
        assert_eq!(parse_literal("abc"), Ok(("", "abc")));
        assert_eq!(parse_literal("(abc)"), Ok(("", "(abc)")));
    }

    #[test]
    fn test_parse_fragment() {
        assert_eq!(
            parse_fragment("abc"),
            Ok(("", StringFragment::Literal("abc")))
        );

        assert_eq!(
            parse_fragment("(abc)"),
            Ok(("", StringFragment::Literal("(abc)")))
        );
    }
}
