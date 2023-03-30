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
    /// Path to the asset to be embed, which can be a binary or a text file
    pub input: String,

    /// Write generated source code in the specified output file instead of stdout
    #[clap(short, long, value_parser)]
    pub output: Option<String>,

    /// Use a specific variable name for the generated content, instead of the input file name
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

    /// Padding value of the generated source code
    #[clap(long, value_parser, default_value_t = 4)]
    pub padding: usize,

    /// Number of byte elements per line
    #[clap(long, value_parser, default_value_t = 16)]
    pub quantity: usize,

    /// Make generated variables mutable
    #[clap(long, value_parser)]
    pub mutable: bool,
}
