use nom::{
    bytes::complete::tag,
    character::complete::{self, multispace1, space1},
    IResult,
};

#[derive(Debug)]
pub struct Object {
    pub number: u32,
    pub revision: u32,
}

pub fn parse_object(input: &str) -> IResult<&str, Object> {
    let (input, number) = complete::u32(input)?;
    let (input, _) = space1(input)?;
    let (input, revision) = complete::u32(input)?;
    let (input, _) = space1(input)?;

    let (input, _) = tag("obj")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("<<")(input)?;
    let (input, _) = multispace1(input)?;

    Ok((input, Object { number, revision }))
}
