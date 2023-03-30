use super::*;

pub fn parse(mut params: Params) -> Result<(), Error> {
    let name = match params.mutable {
        true => params.name.clone(),
        false => params.name.to_uppercase(),
    };

    let brackets = match params.format {
        Format::Hex => "([",
        _ => "(",
    };

    writeln!(params.output, "{} = bytes{}", name, brackets)?;

    write_data(&mut params)?;

    match params.format {
        Format::Hex => writeln!(params.output, "\n])")?,
        _ => writeln!(params.output, "\", encoding='raw_unicode_escape'\n)")?,
    }

    Ok(())
}
