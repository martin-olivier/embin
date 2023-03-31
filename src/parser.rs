use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

use crate::args::{Args, Indent};
use crate::lang::{Input, Params};
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

    if args.quantity == 0 {
        panic!("Quantity parameter must be greater than 0");
    }

    if args.indent == Indent::Tab && args.padding % 4 != 0 {
        panic!("Padding must be a multiple of 4 when using tabs as indentation type");
    }

    let mut input_list = vec![];

    for input in args.input.iter() {
        let (len, file) = match File::open(input) {
            Ok(file) => (
                file.metadata().unwrap().len() as usize,
                BufReader::new(file),
            ),
            Err(e) => panic!("Could not open input file: {}", e),
        };

        let name = match Path::new(&input).file_name() {
            Some(name) => name.to_str().unwrap_or("").replace('.', "_"),
            None => panic!("Could not retrieve input file name"),
        };

        if name.is_empty() {
            panic!("File name cannot be empty");
        }

        if input_list.iter().any(|i: &Input| i.name == name) {
            panic!("Input file names must be unique");
        }

        input_list.push(Input { file, name, len });
    }

    Params {
        input: input_list,
        output,
        mutable: args.mutable,
        format: args.format,
        indent: args.indent,
        padding: args.padding,
        quantity: args.quantity,
    }
}
