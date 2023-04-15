use nom::{
    bytes::complete::{tag, take_until1},
    sequence::delimited,
    IResult,
};

pub fn stream(input: &str) -> IResult<&str, &str> {
    delimited(tag("stream"), take_until1("endstream"), tag("endstream"))(input)
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
            Ok(("", "(random data)"))
        );
    }
}
