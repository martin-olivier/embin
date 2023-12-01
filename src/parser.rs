use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

use crate::args::{Args, Indent};
use crate::lang::{Input, Params};

#[derive(thiserror::Error, Debug)]
pub enum ParserError {
    #[error("Could not open input file \"{0}\": {1}")]
    Input(String, std::io::Error),
    #[error("Could not create output file \"{0}\": {1}")]
    Output(String, std::io::Error),
    #[error("Could not retrieve metadata of input file \"{0}\": {1}")]
    Metadata(String, std::io::Error),
    #[error("Could not retrieve input file name for \"{0}\"")]
    FileName(String),
    #[error("Unique input file names are required, found duplicate \"{0}\"")]
    DuplicateFileName(String),
    #[error("Quantity parameter must be greater than 0")]
    Quantity,
    #[error("Padding must be a multiple of 4 when using tabs as indentation type")]
    Padding,
}

pub fn parse(args: &Args) -> Result<Params, ParserError> {
    let output_buf: BufWriter<Box<dyn Write>> = match args.output {
        Some(ref path) => {
            let file = match File::create(path) {
                Ok(file) => file,
                Err(err) => {
                    return Err(ParserError::Output(path.to_owned(), err));
                }
            };
            BufWriter::new(Box::new(file))
        }
        None => BufWriter::new(Box::new(std::io::stdout())),
    };

    if args.quantity == 0 {
        return Err(ParserError::Quantity);
    }

    if args.indent == Indent::Tab && args.padding % 4 != 0 {
        return Err(ParserError::Padding);
    }

    let mut input_list = vec![];

    for input in args.input.iter() {
        if Path::new(input).is_dir() {
            return Err(ParserError::Input(
                input.to_owned(),
                std::io::Error::new(std::io::ErrorKind::Other, "Input file is a directory"),
            ));
        }

        let (len, file) = match File::open(input) {
            Ok(file) => (
                match file.metadata() {
                    Ok(metadata) => metadata.len() as usize,
                    Err(err) => {
                        return Err(ParserError::Metadata(input.to_owned(), err));
                    }
                },
                BufReader::new(file),
            ),
            Err(err) => {
                return Err(ParserError::Input(input.to_owned(), err));
            }
        };

        let name = match Path::new(&input).file_name() {
            Some(name) => match name.to_str() {
                Some(name) => name.replace(['.', ' '], "_"),
                None => return Err(ParserError::FileName(input.to_owned())),
            },
            None => return Err(ParserError::FileName(input.to_owned())),
        };

        if input_list.iter().any(|i: &Input| i.name == name) {
            return Err(ParserError::DuplicateFileName(name));
        }

        input_list.push(Input { file, name, len });
    }

    Ok(Params {
        input: input_list,
        output: output_buf,
        mutable: args.mutable,
        format: args.format,
        indent: args.indent,
        padding: args.padding,
        quantity: args.quantity,
    })
}
