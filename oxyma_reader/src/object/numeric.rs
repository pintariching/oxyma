use nom::combinator::map;
use nom::number::complete::float;
use nom::IResult;

#[allow(unused)]
#[derive(Debug, PartialEq)]
pub enum Numeric {
    // Int(i32), not yet implemented
    Real(f32),
}

pub fn numeric(input: &str) -> IResult<&str, Numeric> {
    let mut parse_f32 = map(float, Numeric::Real);
    //let parse_i32 = map(character::complete::i32, Numeric::Int);

    parse_f32(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric() {
        assert_eq!(numeric("42"), Ok(("", Numeric::Real(42.))));
        assert_eq!(numeric("-42"), Ok(("", Numeric::Real(-42.))));
        assert_eq!(numeric("3.1415"), Ok(("", Numeric::Real(3.1415))));
        assert_eq!(numeric("-1.2"), Ok(("", Numeric::Real(-1.2))));
    }
}
