use anyhow::Result;
use oxyma_reader::pdf;

#[derive(Debug, PartialEq)]
struct PdfVersion {
    major: u8,
    minor: u8,
}

fn main() -> Result<()> {
    let str = include_str!("example.pdf");
    let (input, pdf) = pdf(str)?;

    dbg!(pdf);
    dbg!(input);

    Ok(())
}
