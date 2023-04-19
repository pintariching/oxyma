use nom::branch::alt;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::IResult;
use std::collections::HashMap;

mod array;
mod boolean;
mod dictionary;
mod indirect;
mod name;
mod null;
mod numeric;
mod stream;
mod string;

use array::array;
use boolean::boolean;
use dictionary::dictionary;
use name::name;
use null::null;
use numeric::{numeric, Numeric};
use stream::stream;
use string::string;

#[derive(Debug, PartialEq)]
pub enum ObjectValue {
    Boolean(bool),
    Numeric(Numeric),
    String(String),
    Name(String),
    Array(Vec<ObjectValue>),
    Dictionary(HashMap<String, ObjectValue>),
    Stream(String),
    Null,
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
            map(stream, ObjectValue::Stream),
            map(null, |_| ObjectValue::Null),
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
