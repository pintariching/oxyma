use nom::branch::alt;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::IResult;
use std::collections::HashMap;

pub mod array;
pub mod boolean;
pub mod dictionary;
pub mod name;
pub mod null;
pub mod numeric;
pub mod reference;
pub mod stream;
pub mod string;

use array::array;
use boolean::boolean;
use dictionary::dictionary;
use name::name;
use null::null;
use numeric::{numeric, Numeric};
use reference::{reference, Reference};
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
    Reference(Reference),
}

pub fn object_value(input: &str) -> IResult<&str, ObjectValue> {
    preceded(
        space0,
        alt((
            map(reference, ObjectValue::Reference),
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
}
