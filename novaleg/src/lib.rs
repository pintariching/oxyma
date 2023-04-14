use nom::IResult;
use object::{object_value, Object};
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
    let (input, value) = object_value(input)?;

    Ok((
        input,
        Pdf {
            version,
            object: Object { value },
        },
    ))
}
