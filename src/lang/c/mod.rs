use super::*;

pub fn parse(mut params: Params) -> Result<(), Error> {
    let accessibility = match params.mutable {
        true => "",
        false => "const ",
    };

    writeln!(
        params.output,
        "{}unsigned char {}[] = {{",
        accessibility, params.name
    )?;

    write_data(&mut params)?;

    writeln!(params.output, "}};")?;

    writeln!(
        params.output,
        "{}unsigned int {}_len = {};",
        accessibility, params.name, params.len
    )?;

    Ok(())
}
