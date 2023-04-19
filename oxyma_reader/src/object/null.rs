use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::IResult;

pub fn null(input: &str) -> IResult<&str, ()> {
    map(tag("null"), |_| ())(input)
}

#[cfg(test)]
mod tests {
    use nom::Finish;

    use super::*;

    #[test]
    fn test_null() {
        assert_eq!(null("null"), Ok(("", ())));
        assert!(null("not null").finish().is_err());
    }
}
