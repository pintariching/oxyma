use std::collections::HashMap;

use nom::branch::alt;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::IResult;

mod array;
mod boolean;
mod dictionary;
mod name;
mod numeric;
mod stream;
mod string;

use array::*;
use boolean::*;
use dictionary::*;
use name::*;
use numeric::*;
use string::*;

#[derive(Debug)]
pub struct Object {
    pub value: ObjectValue,
}

#[derive(Debug, PartialEq)]
pub struct Identifier {
    obj_num: u32,
    gen_num: u32,
}

#[derive(Debug, PartialEq)]
pub enum ObjectValue {
    Boolean(bool),
    Numeric(Numeric),
    String(String),
    Name(String),
    Array(Vec<ObjectValue>),
    Dictionary(HashMap<String, ObjectValue>),
    Stream,
    Null,
    Indirect(Identifier),
}

pub fn object_value(input: &str) -> IResult<&str, ObjectValue> {
    preceded(
        space0,
        alt((
            map(boolean, ObjectValue::Boolean),
            map(numeric, ObjectValue::Numeric),
            map(string, ObjectValue::String),
            map(name, ObjectValue::Name),
            map(array, ObjectValue::Array),
            map(dictionary, ObjectValue::Dictionary),
        )),
    )(input)

    // let (input, number) = complete::u32(input)?;
    // let (input, _) = space1(input)?;
    // let (input, revision) = complete::u32(input)?;
    // let (input, _) = space1(input)?;

    // let (input, _) = tag("obj")(input)?;
    // let (input, _) = multispace1(input)?;
    // let (input, _) = tag("<<")(input)?;
    // let (input, _) = multispace1(input)?;

    // Ok((input, Object { number, revision }))
}
