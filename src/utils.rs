use std::{fmt::Display, io::Write};

pub fn write_semicolon_list<T: Display>(
    writer: &mut impl Write,
    key: &str,
    items: &[T],
) -> std::io::Result<()> {
    write!(writer, "{key}=")?;
    for item in items {
        write!(writer, "{item};")?;
    }
    writeln!(writer)?;
    Ok(())
}
