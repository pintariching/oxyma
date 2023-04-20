mod indirect_object;
mod object;
mod version;

use indirect_object::{indirect_object, IndirectObject};
use nom::multi::many1;
use nom::IResult;
use version::{pdf_version, Version};

#[derive(Debug, PartialEq)]
pub struct Pdf {
    version: Version,
    objects: Vec<IndirectObject>,
}

pub fn pdf(input: &str) -> IResult<&str, Pdf> {
    let (input, version) = pdf_version(input)?;
    let (input, objects) = many1(indirect_object)(input)?;

    Ok((input, Pdf { version, objects }))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::indirect_object::Identifier;
    use crate::object::reference::Reference;
    use crate::object::ObjectValue;

    use super::*;

    #[test]
    fn test_pdf() {
        assert_eq!(
            pdf("%PDF-2.0
1 0 obj
(Brilling)
endobj"),
            Ok((
                "",
                Pdf {
                    version: Version { major: 2, minor: 0 },
                    objects: vec![IndirectObject {
                        ident: Identifier {
                            obj_num: 1,
                            gen_num: 0
                        },
                        value: ObjectValue::String("Brilling".to_string())
                    }]
                }
            ))
        );

        assert_eq!(
            pdf("%PDF-2.0
1 0 obj
<</Type /Catalog /Pages 2 0 R>>
endobj"),
            Ok((
                "",
                Pdf {
                    version: Version { major: 2, minor: 0 },
                    objects: vec![IndirectObject {
                        ident: Identifier {
                            obj_num: 1,
                            gen_num: 0
                        },
                        value: ObjectValue::Dictionary(HashMap::from([
                            ("Type".to_string(), ObjectValue::Name("Catalog".to_string())),
                            (
                                "Pages".to_string(),
                                ObjectValue::Reference(Reference {
                                    obj_num: 2,
                                    gen_num: 0
                                })
                            )
                        ]))
                    }]
                }
            ))
        );
    }
}
