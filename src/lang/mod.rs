pub mod c;
pub mod cpp;
pub mod python;

use crate::args::{Format, Indent};
use std::fs::File;
use std::io::{BufReader, BufWriter, Error, Read, Write};

pub struct Params {
    pub input: BufReader<File>,
    pub output: BufWriter<Box<dyn Write>>,
    pub name: String,
    pub len: usize,
    pub mutable: bool,
    pub format: Format,
    pub indent: Indent,
    pub padding: usize,
    pub quantity: usize,
}

fn write_data(params: &mut Params) -> Result<(), Error> {
    let padding = match params.indent {
        Indent::Space => " ".repeat(params.padding),
        Indent::Tab => "\t".repeat(params.padding),
    };

    for (it, byte) in params.input.by_ref().bytes().enumerate() {
        let byte = byte?;

        let line_begin = it % params.quantity == 0;
        let line_end = it % params.quantity == params.quantity - 1;
        let last = it == params.len - 1;

        if line_begin {
            match params.format {
                Format::Hexa => write!(params.output, "{}", padding)?,
                _ => write!(params.output, "{}\"", padding)?,
            }
        }

        let separator = match last {
            true => "",
            false => ",",
        };

        let space = match line_end || last {
            true => "",
            false => " ",
        };

        match params.format {
            Format::Hexa => write!(params.output, "0x{:02x}{}{}", byte, separator, space)?,
            Format::Octal => write!(params.output, "\\{:03o}", byte as u32)?,
            Format::Char => {
                if (32..=126).contains(&byte) && ![b'"', b'\\', b'?', b':', b'%'].contains(&(byte))
                {
                    write!(params.output, "{}", byte as char)?;
                } else if byte == b'\r' {
                    write!(params.output, "\\r")?;
                } else if byte == b'\n' {
                    write!(params.output, "\\n")?;
                } else if byte == b'\t' {
                    write!(params.output, "\\t")?;
                } else if byte == b'\"' {
                    write!(params.output, "\\\"")?;
                } else if byte == b'\\' {
                    write!(params.output, "\\\\")?;
                } else {
                    write!(params.output, "\\{:03o}", byte as u32)?;
                }
            }
        };

        if line_end && !last {
            match params.format {
                Format::Hexa => writeln!(params.output)?,
                _ => writeln!(params.output, "\"")?,
            }
        }
    }

    Ok(())
}
