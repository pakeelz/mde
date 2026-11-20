use std::{fmt::Display, io::Write};

use crate::{LocaleString, desktop_entry::LocaleStrings};

/// Writes a semicolon-delimited entry list formatted as `KEY=item1;item2;`.
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

/// Writes single-value localized entries (`KEY=...` or `KEY[locale]=...`).
pub fn write_locale_list(
    writer: &mut impl Write,
    key: &str,
    locale_string: &Vec<LocaleString>,
) -> std::io::Result<()> {
    for generic in locale_string {
        if generic.locale.trim().is_empty() {
            writeln!(writer, "{key}={}", generic.value)?;
        } else {
            writeln!(writer, "{key}[{}]={}", generic.locale, generic.value)?;
        }
    }
    Ok(())
}

/// Writes semicolon-delimited localized entries (`KEY=a;b;` or `KEY[locale]=a;b;`).
pub fn write_locales_list(
    writer: &mut impl Write,
    key: &str,
    locale_string: &Vec<LocaleStrings>,
) -> std::io::Result<()> {
    for generic in locale_string {
        if generic.locale.trim().is_empty() {
            write!(writer, "{key}=")?;
            for &value in &generic.values {
                write!(writer, "{};", value)?;
            }
            writeln!(writer)?;
        } else {
            write!(writer, "{key}[{}]=", generic.locale)?;
            for value in &generic.values {
                write!(writer, "{};", value)?;
            }
            writeln!(writer)?;
        }
    }
    Ok(())
}
