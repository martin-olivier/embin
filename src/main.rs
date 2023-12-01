mod args;
mod lang;
mod parser;

use args::{Args, Language};
use clap::Parser;

use colored::Colorize;

fn main() {
    let args = Args::parse();

    let params = match parser::parse(&args) {
        Ok(params) => params,
        Err(e) => terminate(e.to_string()),
    };

    if let Err(e) = match args.lang {
        Language::C => lang::c::parse(params),
        Language::Cpp => lang::cpp::parse(params),
        Language::Python => lang::python::parse(params),
    } {
        terminate(e.to_string());
    }
}

fn terminate(cause: String) -> ! {
    eprintln!("{}", cause.red().bold());
    std::process::exit(1);
}
