mod args;
mod lang;
mod parsing;

use args::{Args, Language};
use clap::Parser;

fn main() {
    let args = Args::parse();
    let params = parsing::parse(&args);

    match args.lang {
        Language::C => lang::c::parse(params),
        Language::Cpp => lang::cpp::parse(params),
        Language::Python => lang::python::parse(params),
    }
    .unwrap();
}
