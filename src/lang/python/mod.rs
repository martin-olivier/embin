use super::*;

pub fn parse(mut params: Params) -> Result<(), Error> {
    let name = match params.mutable {
        true => params.name.clone(),
        false => params.name.to_uppercase(),
    };

    writeln!(params.output, "{} = bytes([", name)?;

    write_data(&mut params)?;

    writeln!(params.output, "])")?;

    Ok(())
}
