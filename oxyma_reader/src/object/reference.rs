use nom::bytes::complete::tag;
use nom::character::complete::{multispace0, space1, u32};
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct Reference {
    pub obj_num: u32,
    pub gen_num: u32,
}

pub fn reference(input: &str) -> IResult<&str, Reference> {
    map(
        tuple((multispace0, u32, space1, u32, space1, tag("R"))),
        |(_, obj_num, _, gen_num, _, _)| Reference { obj_num, gen_num },
    )(input)
}

#[cfg(test)]
mod tests {
    use nom::Finish;

    use super::*;

    #[test]
    fn test_reference() {
        assert_eq!(
            reference("1 0 R"),
            Ok((
                "",
                Reference {
                    obj_num: 1,
                    gen_num: 0
                }
            ))
        );

        assert!(reference("1 0").finish().is_err());
    }
}
