use nom::character::complete::{char, space0};
use nom::multi::separated_list0;
use nom::sequence::{preceded, terminated};
use nom::IResult;

use super::{object_value, ObjectValue};

pub fn array(input: &str) -> IResult<&str, Vec<ObjectValue>> {
    preceded(
        char('['),
        terminated(
            separated_list0(char(' '), object_value),
            preceded(space0, char(']')),
        ),
    )(input)
}

#[cfg(test)]
mod tests {
    use crate::object::numeric::Numeric;

    use super::*;

    #[test]
    fn test_array() {
        assert_eq!(
            array("[549]"),
            Ok(("", vec![ObjectValue::Numeric(Numeric::Real(549.))])),
        );

        assert_eq!(
            array("[549 3.14]"),
            Ok((
                "",
                vec![
                    ObjectValue::Numeric(Numeric::Real(549.)),
                    ObjectValue::Numeric(Numeric::Real(3.14))
                ]
            )),
        );

        assert_eq!(
            array("[549 3.14 false (Ralph) /SomeName]"),
            Ok((
                "",
                vec![
                    ObjectValue::Numeric(Numeric::Real(549.)),
                    ObjectValue::Numeric(Numeric::Real(3.14)),
                    ObjectValue::Boolean(false),
                    ObjectValue::String("Ralph".to_string()),
                    ObjectValue::Name("SomeName".to_string()),
                ]
            )),
        );

        assert_eq!(
            array("[3.14 [false]]"),
            Ok((
                "",
                vec![
                    ObjectValue::Numeric(Numeric::Real(3.14)),
                    ObjectValue::Array(vec![ObjectValue::Boolean(false)])
                ]
            ))
        );
    }
}
