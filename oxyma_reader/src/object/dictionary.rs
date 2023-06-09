use nom::bytes::complete::tag;
use nom::character::complete::{multispace0, multispace1, space1};
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::{preceded, separated_pair, terminated};
use nom::IResult;
use std::collections::HashMap;

use super::{name, object_value, ObjectValue};

fn key_value(input: &str) -> IResult<&str, (String, ObjectValue)> {
    separated_pair(name, space1, object_value)(input)
}

pub fn dictionary(input: &str) -> IResult<&str, HashMap<String, ObjectValue>> {
    preceded(
        preceded(multispace0, preceded(tag("<<"), multispace0)),
        terminated(
            map(separated_list0(multispace1, key_value), |tuple_pairs| {
                tuple_pairs.into_iter().collect()
            }),
            preceded(multispace0, tag(">>")),
        ),
    )(input)
}

#[cfg(test)]
mod tests {
    use crate::object::{numeric::Numeric, reference::Reference};

    use super::*;

    #[test]
    fn test_key_value() {
        assert_eq!(
            key_value("/Pi 3.14"),
            Ok((
                "",
                ("Pi".to_string(), ObjectValue::Numeric(Numeric::Real(3.14)))
            ))
        );

        assert_eq!(
            key_value("/Pi 3.14 /Type /Example"),
            Ok((
                " /Type /Example",
                ("Pi".to_string(), ObjectValue::Numeric(Numeric::Real(3.14)))
            ))
        );
    }

    #[test]
    fn test_dictionary() {
        assert_eq!(
            dictionary("<</Pi 3.14>>"),
            Ok((
                "",
                HashMap::from([("Pi".to_string(), ObjectValue::Numeric(Numeric::Real(3.14)))])
            ))
        );

        assert_eq!(
            dictionary("<< /Pi 3.14 >>"),
            Ok((
                "",
                HashMap::from([("Pi".to_string(), ObjectValue::Numeric(Numeric::Real(3.14)))])
            ))
        );

        assert_eq!(
            dictionary(
                "<<
                    /Type /Example
                    /SubType /DictionaryExample
                >>"
            ),
            Ok((
                "",
                HashMap::from([
                    ("Type".to_string(), ObjectValue::Name("Example".to_string())),
                    (
                        "SubType".to_string(),
                        ObjectValue::Name("DictionaryExample".to_string())
                    )
                ])
            ))
        );

        assert_eq!(
            dictionary("<</Type /Catalog /Pages 2 0 R>>"),
            Ok((
                "",
                HashMap::from([
                    ("Type".to_string(), ObjectValue::Name("Catalog".to_string())),
                    (
                        "Pages".to_string(),
                        ObjectValue::Reference(Reference {
                            obj_num: 2,
                            gen_num: 0
                        })
                    )
                ])
            ))
        );

        assert_eq!(
            dictionary(
                "<<
                /Nested <<
                    /Child 42
                >>
            >>"
            ),
            Ok((
                "",
                HashMap::from([(
                    "Nested".to_string(),
                    ObjectValue::Dictionary(HashMap::from([(
                        "Child".to_string(),
                        ObjectValue::Numeric(Numeric::Real(42.))
                    )]))
                )])
            ))
        );
    }
}
