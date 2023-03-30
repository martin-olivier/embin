use super::*;

pub fn parse(mut params: Params) -> Result<(), Error> {
    let accessibility = match params.mutable {
        true => "",
        false => "constexpr ",
    };

    let storage = match params.format {
        Format::Hex => "array",
        _ => "string_view",
    };

    let template = match params.format {
        Format::Hex => format!("<unsigned char, {}>", params.len),
        _ => "".to_string(),
    };

    let brackets = match params.format {
        Format::Hex => " {",
        _ => "(",
    };

    writeln!(params.output, "#include <{}>\n", storage)?;

    writeln!(
        params.output,
        "{}std::{}{} {}{}",
        accessibility, storage, template, params.name, brackets
    )?;

    write_data(&mut params)?;

    match params.format {
        Format::Hex => writeln!(params.output, "\n}};")?,
        _ => writeln!(params.output, "\", {}\n);", params.len)?,
    }

    Ok(())
}
