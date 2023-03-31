use super::*;

pub fn parse(mut params: Params) -> Result<(), Error> {
    for idx in 0..params.input.len() {
        let name = match params.mutable {
            true => params.input[idx].name.clone(),
            false => params.input[idx].name.to_uppercase(),
        };

        let brackets = match params.format {
            Format::Hex => "([",
            _ => "(",
        };

        writeln!(params.output, "{} = bytes{}", name, brackets)?;

        write_data(&mut params, idx)?;

        match params.format {
            Format::Hex => writeln!(params.output, "\n])")?,
            _ => writeln!(params.output, "\", encoding='raw_unicode_escape'\n)")?,
        }

        if idx != params.input.len() - 1 {
            writeln!(params.output)?;
        }
    }

    Ok(())
}
