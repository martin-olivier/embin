use super::*;

pub fn parse(mut params: Params) -> Result<(), Error> {
    let accessibility = match params.mutable {
        true => "",
        false => "constexpr ",
    };

    writeln!(params.output, "#include <array>\n")?;
    writeln!(
        params.output,
        "{}std::array<unsigned char, {}> {} = {{",
        accessibility, params.len, params.name
    )?;

    write_data(&mut params)?;

    writeln!(params.output, "}};")?;

    Ok(())
}
