mod args;
mod lang;
mod parser;

use args::{Args, Language};
use clap::Parser;

fn main() {
    let args = Args::parse();
    let params = parser::parse(&args);

    match args.lang {
        Language::C => lang::c::parse(params),
        Language::Cpp => lang::cpp::parse(params),
        Language::Python => lang::python::parse(params),
    }
    .unwrap();
}
