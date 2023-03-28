mod args;
mod lang;

use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

use args::{Args, Language};
use clap::Parser;
use colored::Colorize;
use lang::Params;

fn main() {
    let args = Args::parse();

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
        Some(name) => name,
        None => match Path::new(&args.input).file_name() {
            Some(name) => name.to_str().unwrap().replace('.', "_"),
            None => panic!("Could not retrieve input file name"),
        },
    };

    let (len, input) = match File::open(args.input) {
        Ok(file) => (file.metadata().unwrap().len(), BufReader::new(file)),
        Err(e) => panic!("Could not open input file: {}", e),
    };

    let output: BufWriter<Box<dyn Write>> = match args.output {
        Some(path) => {
            let file = match File::create(path) {
                Ok(file) => file,
                Err(e) => panic!("Could not create output file: {}", e),
            };
            BufWriter::new(Box::new(file))
        }
        None => BufWriter::new(Box::new(std::io::stdout())),
    };

    let params = Params {
        input,
        output,
        name,
        len: len as usize,
        mutable: args.mutable,
        padding: args.padding,
        quantity: args.quantity,
    };

    match args.lang {
        Language::C => lang::c::parse(params),
        Language::Cpp => lang::cpp::parse(params),
        Language::Python => lang::python::parse(params),
    }
    .unwrap();
}
