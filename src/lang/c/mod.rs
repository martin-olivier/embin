use super::*;

pub fn parse(mut params: Params) -> Result<(), Error> {
    for idx in 0..params.input.len() {
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
            accessibility, params.input[idx].name, brackets,
        )?;

        write_data(&mut params, idx)?;

        match params.format {
            Format::Hex => writeln!(params.output, "\n}};")?,
            _ => writeln!(params.output, "\";")?,
        }

        writeln!(
            params.output,
            "{}unsigned int {}_len = {};",
            accessibility, params.input[idx].name, params.input[idx].len
        )?;

        if idx != params.input.len() - 1 {
            writeln!(params.output)?;
        }
    }

    Ok(())
}
