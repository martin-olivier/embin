pub mod c;
pub mod cpp;
pub mod python;

use std::fs::File;
use std::io::{BufReader, BufWriter, Error, Read, Write};

pub struct Params {
    pub input: BufReader<File>,
    pub output: BufWriter<Box<dyn Write>>,
    pub name: String,
    pub len: usize,
    pub mutable: bool,
    pub padding: usize,
    pub quantity: usize,
}

fn write_data(params: &mut Params) -> Result<(), Error> {
    let padding = " ".repeat(params.padding);

    for (it, byte) in params.input.by_ref().bytes().enumerate() {
        let byte = byte?;

        let begin = it % params.quantity == 0;
        let end = it % params.quantity == params.quantity - 1;
        let last = it == params.len - 1;

        if begin {
            write!(params.output, "{}", padding)?;
        }

        let separator = match last {
            true => "",
            false => ",",
        };

        let space = match end || last {
            true => "",
            false => " ",
        };

        write!(params.output, "0x{:02x}{}{}", byte, separator, space)?;

        if end || last {
            writeln!(params.output)?;
        }
    }

    Ok(())
}
