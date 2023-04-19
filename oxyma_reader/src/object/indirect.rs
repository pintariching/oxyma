use nom::bytes::complete::tag;
use nom::character;
use nom::character::complete::{multispace0, space0, space1};
use nom::sequence::{delimited, preceded};
use nom::IResult;

use crate::object::{object_value, ObjectValue};

#[derive(Debug, PartialEq)]
pub struct IndirectObject {
    pub ident: Identifier,
    pub value: ObjectValue,
}

#[derive(Debug, PartialEq)]
pub struct Identifier {
    obj_num: u32,
    gen_num: u32,
}

fn identifier(input: &str) -> IResult<&str, Identifier> {
    let (input, obj_num) = preceded(multispace0, character::complete::u32)(input)?;
    let (input, gen_num) = preceded(space1, character::complete::u32)(input)?;

    Ok((input, Identifier { obj_num, gen_num }))
}

fn indirect_object(input: &str) -> IResult<&str, IndirectObject> {
    let (input, ident) = identifier(input)?;
    let (input, value) = preceded(
        space1,
        delimited(tag("obj"), object_value, preceded(space0, tag("endobj"))),
    )(input)?;

    Ok((input, IndirectObject { ident, value }))
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
