use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, multispace1, space1},
    combinator::{map, value},
    number, IResult,
};

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

#[derive(Debug)]
pub enum Numeric {
    Int(i32),
    UInt(u32),
    Float(f32),
}

pub fn parse_object(input: &str) -> IResult<&str, Object> {
    let (input, object_type) = alt((map(boolean, ObjectType::Boolean), string))(input)?;
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

pub fn boolean(input: &str) -> IResult<&str, bool> {
    let parse_true = value(true, tag("true"));
    let parse_false = value(false, tag("false"));

    alt((parse_true, parse_false))(input)
}

pub fn numeric(input: &str) -> IResult<&str, Numeric> {
    let parse_f32 = map(number::complete::le_f32, Numeric::Float);
    // let parse_i32 = map(number::complete::le_i32, Numeric::Int);
    // let (input, numeric) = alt()
    todo!()
}

pub fn string(input: &str) -> IResult<&str, ObjectType> {
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
