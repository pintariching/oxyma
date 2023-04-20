use nom::bytes::complete::tag;
use nom::character::complete::{multispace0, space1, u32};
use nom::combinator::map;
use nom::sequence::{delimited, preceded, tuple};
use nom::IResult;

use crate::object::{object_value, ObjectValue};

#[derive(Debug, PartialEq)]
pub struct IndirectObject {
    pub ident: Identifier,
    pub value: ObjectValue,
}

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub obj_num: u32,
    pub gen_num: u32,
}

fn identifier(input: &str) -> IResult<&str, Identifier> {
    map(
        tuple((multispace0, u32, space1, u32)),
        |(_, obj_num, _, gen_num)| Identifier { obj_num, gen_num },
    )(input)
}

pub fn indirect_object(input: &str) -> IResult<&str, IndirectObject> {
    map(
        tuple((
            identifier,
            space1,
            delimited(
                tag("obj"),
                object_value,
                preceded(multispace0, tag("endobj")),
            ),
        )),
        |(ident, _, value)| IndirectObject { ident, value },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identifier() {
        assert_eq!(
            identifier("1 0"),
            Ok((
                "",
                Identifier {
                    obj_num: 1,
                    gen_num: 0
                }
            ))
        );
    }

    #[test]
    fn test_indirect_object() {
        assert_eq!(
            indirect_object("1 0 obj (Brilling) endobj"),
            Ok((
                "",
                IndirectObject {
                    ident: Identifier {
                        obj_num: 1,
                        gen_num: 0
                    },
                    value: ObjectValue::String("Brilling".to_string())
                }
            ))
        );
    }
}
