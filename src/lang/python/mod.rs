use super::*;

pub fn parse(mut params: Params) -> Result<(), Error> {
    let name = match params.mutable {
        true => params.name.clone(),
        false => params.name.to_uppercase(),
    };

    let brackets = match params.format {
        Format::Hexa => "([",
        _ => "(",
    };

    writeln!(params.output, "{} = bytes{}", name, brackets)?;

    write_data(&mut params)?;

    match params.format {
        Format::Hexa => writeln!(params.output, "\n])")?,
        _ => writeln!(params.output, "\"\n)")?,
    }

    Ok(())
}
