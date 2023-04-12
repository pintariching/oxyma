use nom::bytes::complete::tag;
use nom::character::complete::{self, multispace0};
use nom::combinator::verify;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

pub fn pdf_version(input: &str) -> IResult<&str, Version> {
    let (input, _) = multispace0(input)?;

    let (input, _) = tag("%PDF-")(input)?;

    let (input, major) = verify(|i| complete::u8(i), |m: &u8| *m == 2)(input)?;
    let (input, _) = tag(".")(input)?;
    let (input, minor) = verify(|i| complete::u8(i), |m: &u8| *m == 0)(input)?;

    let (input, _) = multispace0(input)?;

    Ok((input, Version { major, minor }))
}

#[cfg(test)]
mod tests {
    use nom::Finish;

    use super::*;

    #[test]
    fn test_pdf_version_failing() {
        assert!(pdf_version("%PDF-1.0").finish().is_err());
        assert!(pdf_version("%PDF-1.7").finish().is_err());
        assert!(pdf_version("%PDF-2.1").finish().is_err());
    }

    #[test]
    fn test_pdf_version() {
        assert_eq!(
            pdf_version("%PDF-2.0"),
            Ok((
                "",
                PdfVersion {
                    major: 2u8,
                    minor: 0u8
                }
            ))
        );

        assert_eq!(
            pdf_version("%PDF-2.0 "),
            Ok((
                "",
                PdfVersion {
                    major: 2u8,
                    minor: 0u8
                }
            ))
        );

        assert_eq!(
            pdf_version(" %PDF-2.0"),
            Ok((
                "",
                PdfVersion {
                    major: 2u8,
                    minor: 0u8
                }
            ))
        );

        assert_eq!(
            pdf_version(
                " 
            %PDF-2.0"
            ),
            Ok((
                "",
                PdfVersion {
                    major: 2u8,
                    minor: 0u8
                }
            ))
        );

        assert_eq!(
            pdf_version(
                "%PDF-2.0 
            "
            ),
            Ok((
                "",
                PdfVersion {
                    major: 2u8,
                    minor: 0u8
                }
            ))
        );
    }
}
