use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;

mod boolean;
mod name;
mod numeric;
mod string;

use boolean::*;
use name::*;
use numeric::*;
use string::*;

#[derive(Debug)]
pub struct Object {
    object_type: ObjectType,
}

#[derive(Debug)]
pub struct Identifier {
    obj_num: u32,
    gen_num: u32,
}

#[derive(Debug)]
pub enum ObjectType {
    Boolean(bool),
    Numeric(Numeric),
    String(String),
    Name,
    Array,
    Dictionary,
    Stream,
    Null,
    Indirect(Identifier),
}

pub fn parse_object(input: &str) -> IResult<&str, Object> {
    let (input, object_type) = alt((
        map(boolean, ObjectType::Boolean),
        map(numeric, ObjectType::Numeric),
        map(string, ObjectType::String),
    ))(input)?;

    // let (input, number) = complete::u32(input)?;
    // let (input, _) = space1(input)?;
    // let (input, revision) = complete::u32(input)?;
    // let (input, _) = space1(input)?;

    // let (input, _) = tag("obj")(input)?;
    // let (input, _) = multispace1(input)?;
    // let (input, _) = tag("<<")(input)?;
    // let (input, _) = multispace1(input)?;

    // Ok((input, Object { number, revision }))
    todo!()
}

pub fn name(input: &str) -> IResult<&str, ObjectType> {
    todo!()
}

pub fn array(input: &str) -> IResult<&str, ObjectType> {
    todo!()
}

pub fn dictionary(input: &str) -> IResult<&str, ObjectType> {
    todo!()
}
