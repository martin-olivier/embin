use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

use crate::args::Args;
use crate::lang::Params;
use colored::Colorize;

pub fn parse(args: &Args) -> Params {
    std::panic::set_hook(Box::new(|err| {
        if let Some(msg) = err.payload().downcast_ref::<&str>() {
            println!("{} {}", "Error:".bold().red(), msg.bold().red());
        } else if let Some(msg) = err.payload().downcast_ref::<String>() {
            println!("{} {}", "Error:".bold().red(), msg.bold().red());
        } else {
            println!("{} {}", "Error:".bold().red(), err);
        }
        std::process::exit(1);
    }));

    let name = match args.name {
        Some(ref name) => name.clone(),
        None => match Path::new(&args.input).file_name() {
            Some(name) => name.to_str().unwrap_or("").replace('.', "_"),
            None => panic!("Could not retrieve input file name"),
        },
    };

    if name.is_empty() {
        panic!("File name cannot be empty");
    }

    let (len, input) = match File::open(&args.input) {
        Ok(file) => (file.metadata().unwrap().len(), BufReader::new(file)),
        Err(e) => panic!("Could not open input file: {}", e),
    };

    let output: BufWriter<Box<dyn Write>> = match args.output {
        Some(ref path) => {
            let file = match File::create(path) {
                Ok(file) => file,
                Err(e) => panic!("Could not create output file: {}", e),
            };
            BufWriter::new(Box::new(file))
        }
        None => BufWriter::new(Box::new(std::io::stdout())),
    };

    Params {
        input,
        output,
        name,
        len: len as usize,
        mutable: args.mutable,
        format: args.format,
        indent: args.indent,
        padding: args.padding,
        quantity: args.quantity,
    }
}
