use super::*;

pub fn parse(mut params: Params) -> Result<(), Error> {
    let accessibility = match params.mutable {
        true => "",
        false => "constexpr ",
    };

    let storage = match params.format {
        Format::Hexa => "array",
        _ => "string_view",
    };

    let template = match params.format {
        Format::Hexa => format!("<unsigned char, {}>", params.len),
        _ => "".to_string(),
    };

    let brackets = match params.format {
        Format::Hexa => " {",
        _ => "",
    };

    writeln!(params.output, "#include <{}>\n", storage)?;

    writeln!(
        params.output,
        "{}std::{}{} {} ={}",
        accessibility, storage, template, params.name, brackets
    )?;

    write_data(&mut params)?;

    match params.format {
        Format::Hexa => writeln!(params.output, "\n}};")?,
        _ => writeln!(params.output, "\";")?,
    }

    Ok(())
}
