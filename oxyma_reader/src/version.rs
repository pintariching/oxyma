use nom::bytes::complete::tag;
use nom::character::complete::{self, multispace0};
use nom::combinator::verify;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

pub fn pdf_version(input: &str) -> IResult<&str, Version> {
    let (input, _) = multispace0(input)?;

    let (input, _) = tag("%PDF-")(input)?;

    let (input, major) = verify(|i| complete::u8(i), |m: &u8| *m == 2)(input)?;
    let (input, _) = tag(".")(input)?;
    let (input, minor) = verify(|i| complete::u8(i), |m: &u8| *m == 0)(input)?;

    let (input, _) = multispace0(input)?;

    Ok((input, Version { major, minor }))
}
