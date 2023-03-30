use super::*;

pub fn parse(mut params: Params) -> Result<(), Error> {
    let accessibility = match params.mutable {
        true => "",
        false => "const ",
    };

    let brackets = match params.format {
        Format::Hex => " {",
        _ => "",
    };

    writeln!(
        params.output,
        "{}unsigned char {}[] ={}",
        accessibility, params.name, brackets,
    )?;

    write_data(&mut params)?;

    match params.format {
        Format::Hex => writeln!(params.output, "\n}};")?,
        _ => writeln!(params.output, "\";")?,
    }

    writeln!(
        params.output,
        "{}unsigned int {}_len = {};",
        accessibility, params.name, params.len
    )?;

    Ok(())
}
