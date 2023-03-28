use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Language {
    C,
    Cpp,
    Python,
}

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]

pub struct Args {
    /// Language of the generated source code
    #[arg(value_enum)]
    #[clap(long, value_parser)]
    pub lang: Language,

    /// Binary or text input file
    #[clap(short, long, value_parser)]
    pub input: String,

    /// Source code output file, if not specified, the output will be printed to stdout
    #[clap(short, long, value_parser)]
    pub output: Option<String>,

    /// Name of the variable, if not specified, the name will be the same as the input file
    #[clap(short, long, value_parser)]
    pub name: Option<String>,

    /// Make generated variables mutable
    #[clap(long, value_parser)]
    pub mutable: bool,

    /// Set the padding of the generated source code
    #[clap(long, value_parser, default_value_t = 2)]
    pub padding: usize,

    /// For every N byte, append a newline
    #[clap(long, value_parser, default_value_t = 12)]
    pub quantity: usize,
}
