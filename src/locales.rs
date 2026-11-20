use crate::LocaleString;

/// Converts a plain string, single [`LocaleString`], or list of [`LocaleString`]s
/// into a `Vec<LocaleString>` for builder methods that accept localized values.
pub trait ToLocale<'a> {
    /// Converts `self` into a vector of [`LocaleString`] entries.
    fn to_locale(self) -> Vec<LocaleString<'a>>;
}

impl<'a> ToLocale<'a> for &'a str {
    /// Wraps a plain string slice as an unlocalized [`LocaleString`] (`locale: ""`).
    fn to_locale(self) -> Vec<LocaleString<'a>> {
        vec![LocaleString::new("", self)]
    }
}

impl<'a> ToLocale<'a> for LocaleString<'a> {
    /// Wraps a single [`LocaleString`] into a one-element vector.
    fn to_locale(self) -> Vec<LocaleString<'a>> {
        vec![LocaleString::new(self.locale, self.value)]
    }
}

impl<'a> ToLocale<'a> for Vec<LocaleString<'a>> {
    /// Returns the vector of [`LocaleString`] entries as-is.
    fn to_locale(self) -> Vec<LocaleString<'a>> {
        self
    }
}
