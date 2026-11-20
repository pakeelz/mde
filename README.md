# mde

A Rust library for building and writing Freedesktop Desktop Entry (`.desktop`) files on Linux.

## Features

- Builder API via [`DesktopEntry`] and [`DesktopAction`] for configuring entry metadata, paths, desktop environments, categories, and desktop actions.
- Localization support via [`LocaleString`], [`LocaleStrings`], and [`ToLocale`] for localized keys (`Name[locale]`, `GenericName[locale]`, `Comment[locale]`, `Keywords[locale]`).
- Typed enums for entry [`Types`], menu [`DesktopCategories`], and [`DesktopEnvironment`] targets (including Wayland compositors and custom values).
- Buffered file output via [`DesktopEntryFile`].

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mde = "0.1.1"
```

or run:
```bash
cargo add mde
```

## Usage

Basic Example
```rust
use std::path::Path;
use mde::{
    DesktopAction, DesktopEntry, DesktopEntryFile, DesktopEnvironment, Icon, Types,
};

fn main() -> std::io::Result<()> {
    let entry = DesktopEntry::new("Pomotimer")
        .types(Types::APPLICATION)
        .version("v0.0.1")
        .comment("Focus timer for Linux desktop sessions")
        .icon(Icon::IconPath(Path::new("/home/user/some_icon.jpg")))
        .try_exec("/usr/local/bin/pomotimer")
        .only_show_in(vec![
            DesktopEnvironment::HYPRLAND,
            DesktopEnvironment::MANGOWM,
            DesktopEnvironment::CINNAMON,
        ])
        .not_show_in(vec![DesktopEnvironment::GNOME])
        .exec("/usr/local/bin/pomotimer %F")
        .mime_types(vec!["text/html", "text/markdown"])
        .action(vec![
            DesktopAction::new("start-focus", "Start Focus Session")
                .icon(Icon::IconPath(Path::new("/home/user/some_icon.jpg")))
                .exec("/usr/local/bin/pomotimer --focus"),
            DesktopAction::new("new-window", "New Window"),
        ]);

    DesktopEntryFile::new(entry).create("my_apps.desktop")?;
    Ok(())
}
```

Localized Example
```rust
use std::path::Path;
use mde::{
    DesktopAction, DesktopEntry, DesktopEntryFile, DesktopEnvironment, Icon, LocaleString,
    LocaleStrings, Types,
};

fn main() -> std::io::Result<()> {
    let entry = DesktopEntry::new(vec![
        LocaleString::new("", "My App"),
        LocaleString::new("id", "Applikasi ku"),
        LocaleString::new("fr", "mon application"),
        LocaleString::new("ja", "私のアプリ"),
    ])
    .types(Types::APPLICATION)
    .version("v0.0.1")
    .comments(vec![
        LocaleString::new("", "This is basic apps"),
        LocaleString::new("fr", "Ce sont des applications de base"),
        LocaleString::new("id", "Ini adalah apps biasa"),
    ])
    .icon(Icon::IconString("pomotimer"))
    .generic_names(vec![
        LocaleString::new("", "Utility"),
        LocaleString::new("id", "Utilitas"),
        LocaleString::new("fr", "Utilitaire"),
        LocaleString::new("ja", "ユーティリティ"),
    ])
    .try_exec("/usr/local/bin/pomotimer")
    .only_show_in(vec![
        DesktopEnvironment::HYPRLAND,
        DesktopEnvironment::MANGOWM,
        DesktopEnvironment::CINNAMON,
    ])
    .not_show_in(vec![DesktopEnvironment::GNOME])
    .exec("/usr/local/bin/some_bin %F")
    .mime_types(vec!["text/html", "text/markdown"])
    .keywords(vec![
        LocaleStrings::new("", vec!["Utility", "Browser", "Explorer"]),
        LocaleStrings::new("id", vec!["Utility", "Browser", "Explorer"]),
        LocaleStrings::new("fr", vec!["Utilitaire", "navigateur", "explorateur"]),
    ])
    .action(vec![
        DesktopAction::new("start-focus", "Start Focus Session")
            .icon(Icon::IconPath(Path::new("/home/user/some_icon.jpg")))
            .exec("/usr/local/bin/some_bin"),
        DesktopAction::new("new-window", "New Window"),
        DesktopAction::new(
            "localized-action",
            vec![
                LocaleString::new("", "Just Test 3"),
                LocaleString::new("id", "Hanya Test 3"),
                LocaleString::new("fr", "il suffit d'en tester 3"),
                LocaleString::new("ja", "テスト3だけ"),
            ],
        ),
    ]);

    DesktopEntryFile::new(entry).create("my_app.desktop")?;
    Ok(())
}
```

Notes on Builder Methods

- Calling singular and plural pairs that target the same underlying field—such as comment and comments, generic_name and generic_names, or keyword and keywords—will overwrite the earlier value.

- Use "" as the locale tag when you want to emit the default unlocalized key alongside localized entries.

## License

Licensed under the MIT license. See LICENSE for details.
