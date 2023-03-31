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

    let brackets = match params.format {
        Format::Hex => " {",
        _ => "(",
    };

    writeln!(params.output, "#include <{}>\n", storage)?;

    for idx in 0..params.input.len() {
        let template = match params.format {
            Format::Hex => format!("<unsigned char, {}>", params.input[idx].len),
            _ => "".to_string(),
        };

        writeln!(
            params.output,
            "{}std::{}{} {}{}",
            accessibility, storage, template, params.input[idx].name, brackets
        )?;

        write_data(&mut params, idx)?;

        match params.format {
            Format::Hex => writeln!(params.output, "\n}};")?,
            _ => writeln!(params.output, "\", {}\n);", params.input[idx].len)?,
        }

        if idx != params.input.len() - 1 {
            writeln!(params.output)?;
        }
    }

    Ok(())
}
