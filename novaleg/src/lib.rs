use nom::IResult;
use object::{parse_object, Object};
use version::Version;

use crate::version::pdf_version;

mod object;
mod version;

#[derive(Debug)]
pub struct Pdf {
    version: Version,
    object: Object,
}

pub fn pdf_parser(input: &str) -> IResult<&str, Pdf> {
    let (input, version) = pdf_version(input)?;
    let (input, object) = parse_object(input)?;

    Ok((input, Pdf { version, object }))
}
