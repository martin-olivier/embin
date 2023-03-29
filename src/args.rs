use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Language {
    C,
    Cpp,
    Python,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Indent {
    Space,
    Tab,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Format {
    Hexa,
    Octal,
    Char,
}

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]

pub struct Args {
    /// Input file, which can be a binary file or a text file
    pub input: String,

    /// Output file, if not specified, the output will be printed to stdout
    #[clap(short, long, value_parser)]
    pub output: Option<String>,

    /// Name of the variable, if not specified, the name will be the same as the input file
    #[clap(short, long, value_parser)]
    pub name: Option<String>,

    /// Language of the generated source code
    #[arg(value_enum)]
    #[clap(long, value_parser, default_value_t = Language::C)]
    pub lang: Language,

    /// Format of the generated source code
    #[arg(value_enum)]
    #[clap(long, value_parser, default_value_t = Format::Hexa)]
    pub format: Format,

    /// Indentation type of the generated source code
    #[arg(value_enum)]
    #[clap(long, value_parser, default_value_t = Indent::Space)]
    pub indent: Indent,

    /// Set the padding of the generated source code
    #[clap(long, value_parser, default_value_t = 4)]
    pub padding: usize,

    /// Set the number of elements per line
    #[clap(long, value_parser, default_value_t = 16)]
    pub quantity: usize,

    /// Make generated variables mutable
    #[clap(long, value_parser)]
    pub mutable: bool,
}
