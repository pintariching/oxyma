use anyhow::Result;
use novaleg::pdf_parser;

#[derive(Debug, PartialEq)]
struct PdfVersion {
    major: u8,
    minor: u8,
}

fn main() -> Result<()> {
    let pdf = include_str!("example.pdf");
    let (input, pdf) = pdf_parser(pdf)?;

    dbg!(pdf);
    dbg!(input);

    Ok(())
}
