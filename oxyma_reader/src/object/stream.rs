use nom::bytes::complete::{tag, take_until1};
use nom::combinator::map;
use nom::sequence::delimited;
use nom::IResult;

pub fn stream(input: &str) -> IResult<&str, String> {
    delimited(
        tag("stream"),
        map(take_until1("endstream"), String::from),
        tag("endstream"),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream() {
        assert_eq!(
            stream(
                "stream
        (random data)
        endstream"
            ),
            Ok((
                "",
                "
        (random data)
        "
                .to_string()
            ))
        );
    }
}
