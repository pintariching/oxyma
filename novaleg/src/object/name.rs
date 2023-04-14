use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take, take_till};
use nom::character::complete::char;
use nom::combinator::{map, map_opt, map_res, verify};
use nom::multi::fold_many0;
use nom::sequence::{delimited, preceded};
use nom::IResult;

fn hex_code(input: &str) -> IResult<&str, char> {
    let take_2 = take(2usize);
    let parse_u32 = map_res(take_2, |hex| u32::from_str_radix(hex, 16));

    map_opt(parse_u32, char::from_u32)(input)
}

fn parse_escaped_char(input: &str) -> IResult<&str, char> {
    preceded(char('#'), hex_code)(input)
}

#[derive(Debug, PartialEq)]
enum NameFragment<'a> {
    Literal(&'a str),
    EscapedChar(char),
}

fn parse_string(input: &str) -> IResult<&str, &str> {
    let hash_space = is_not("# []");

    verify(hash_space, |s: &str| !s.is_empty())(input)
}

fn parse_fragment(input: &str) -> IResult<&str, NameFragment> {
    alt((
        map(parse_string, NameFragment::Literal),
        map(parse_escaped_char, NameFragment::EscapedChar),
    ))(input)
}

pub fn name(input: &str) -> IResult<&str, String> {
    let build_string = fold_many0(parse_fragment, String::new, |mut string, fragment| {
        match fragment {
            NameFragment::Literal(l) => string.push_str(l),
            NameFragment::EscapedChar(c) => string.push(c),
        }
        string
    });

    delimited(
        char('/'),
        build_string,
        alt((tag(" "), take_till(|c| c == ']' || c == '['))),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_code() {
        assert_eq!(hex_code("42"), Ok(("", 'B')));
        assert_eq!(hex_code("3E"), Ok(("", '>')));
        assert_eq!(hex_code("4F3E"), Ok(("3E", 'O')));
    }

    #[test]
    fn test_parse_escaped_char() {
        assert_eq!(parse_escaped_char("#42"), Ok(("", 'B')));
        assert_eq!(parse_escaped_char("#3E"), Ok(("", '>')));
        assert_eq!(parse_escaped_char("#4F3E"), Ok(("3E", 'O')));
    }

    #[test]
    fn test_parse_fragment() {
        assert_eq!(
            parse_fragment("abc"),
            Ok(("", NameFragment::Literal("abc")))
        );
        assert_eq!(
            parse_fragment("abc#42"),
            Ok(("#42", NameFragment::Literal("abc")))
        );
        assert_eq!(
            parse_fragment("#42"),
            Ok(("", NameFragment::EscapedChar('B')))
        );
        assert_eq!(
            parse_fragment("#42abc"),
            Ok(("abc", NameFragment::EscapedChar('B')))
        );
        assert_eq!(
            parse_fragment("abc /def"),
            Ok((" /def", NameFragment::Literal("abc")))
        );
        assert_eq!(
            parse_fragment("abc]"),
            Ok(("]", NameFragment::Literal("abc")))
        );
    }

    #[test]
    fn test_name() {
        assert_eq!(name("/abc"), Ok(("", "abc".to_string())));
        assert_eq!(name("/abc#42"), Ok(("", "abcB".to_string())));
        assert_eq!(name("/#42abc"), Ok(("", "Babc".to_string())));
        assert_eq!(name("/abc /def"), Ok(("/def", "abc".to_string())));
        assert_eq!(name("/abc]"), Ok(("]", "abc".to_string())));
        assert_eq!(name("/abc["), Ok(("[", "abc".to_string())));

        // Examples taken from the PDF 2.0 spec - table 4, page 28-29
        assert_eq!(name("/Name1"), Ok(("", "Name1".to_string())));
        assert_eq!(
            name("/ASomewhatLongerName"),
            Ok(("", "ASomewhatLongerName".to_string()))
        );
        assert_eq!(
            name("/A;Name_With-Various***Characters?"),
            Ok(("", "A;Name_With-Various***Characters?".to_string()))
        );
        assert_eq!(name("/@pattern"), Ok(("", "@pattern".to_string())));
        assert_eq!(name("/.notdef"), Ok(("", ".notdef".to_string())));
        assert_eq!(name("/Lime#20Green"), Ok(("", "Lime Green".to_string())));
        assert_eq!(
            name("/paired#28#29parentheses"),
            Ok(("", "paired()parentheses".to_string()))
        );
        assert_eq!(
            name("/The_Key_of_F#23_Minor"),
            Ok(("", "The_Key_of_F#_Minor".to_string()))
        );
        assert_eq!(name("/A#42"), Ok(("", "AB".to_string())));
    }
}
