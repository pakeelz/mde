//! Builder and writer for Freedesktop Desktop Entry (`.desktop`) files on Linux.
//!
//! This crate provides a builder API for defining desktop entry metadata,
//! desktop actions, target desktop environments, and categories, then writing
//! the resulting `.desktop` file to disk.
//!
//! # Simple Examples
//!
//! ```no_run
//! use std::path::Path;
//! use mde::{
//!     DesktopAction, DesktopEntry, DesktopEntryFile, DesktopEnvironment, Icon, Types,
//! };
//!
//! fn main() -> std::io::Result<()> {
//!     let entry = DesktopEntry::new("Pomotimer")
//!         .types(Types::APPLICATION)
//!         .version("v0.0.1")
//!         .comment("Focus timer for Linux desktop sessions")
//!         .icon(Icon::IconPath(Path::new("/home/user/some_icon.jpg")))
//!         .try_exec("/usr/local/bin/pomotimer")
//!         .only_show_in(vec![
//!             DesktopEnvironment::HYPRLAND,
//!             DesktopEnvironment::MANGOWM,
//!             DesktopEnvironment::CINNAMON,
//!         ])
//!         .not_show_in(vec![DesktopEnvironment::GNOME])
//!         .exec("/usr/local/bin/pomotimer %F")
//!         .mime_types(vec!["text/html", "text/markdown"])
//!         .action(vec![
//!             DesktopAction::new("start-focus", "Start Focus Session")
//!                 .icon(Icon::IconPath(Path::new("/home/user/some_icon.jpg")))
//!                 .exec("/usr/local/bin/pomotimer --focus"),
//!             DesktopAction::new("new-window", "New Window"),
//!         ]);
//!
//!     DesktopEntryFile::new(entry).create("my_apps.desktop")?;
//!     Ok(())
//! }
//! ```
//!
//! Generated `my_apps.desktop`:
//!
//! ```ini
//! [Desktop Entry]
//! Type=Application
//! Name=Pomotimer
//! Version=v0.0.1
//! Comment=Focus timer for Linux desktop sessions
//! Icon=/home/user/some_icon.jpg
//! OnlyShowIn=Hyprland;Mangowm;Cinnamon;
//! NotShowIn=Gnome;
//! TryExec=/usr/local/bin/pomotimer
//! Exec=/usr/local/bin/pomotimer %F
//! MimeType=text/html;text/markdown;
//!
//! [Desktop Action start-focus]
//! Name=Start Focus Session
//! Icon=/home/user/some_icon.jpg
//! Exec=/usr/local/bin/pomotimer --focus
//!
//! [Desktop Action new-window]
//! Name=New Window
//! ```
//!
//! # Full Examples
//!
//! ```no_run
//! use std::path::Path;
//! use mde::{
//!     DesktopAction, DesktopEntry, DesktopEntryFile, DesktopEnvironment, Icon, LocaleString,
//!     LocaleStrings, Types,
//! };
//!
//! fn main() -> std::io::Result<()> {
//!     let entry = DesktopEntry::new(vec![
//!         LocaleString::new("", "My App"),
//!         LocaleString::new("id", "Applikasi ku"),
//!         LocaleString::new("fr", "mon application"),
//!         LocaleString::new("ja", "私のアプリ"),
//!     ])
//!     .types(Types::APPLICATION)
//!     .version("v0.0.1")
//!     .comments(vec![
//!         LocaleString::new("", "This is basic apps"),
//!         LocaleString::new("fr", "Ce sont des applications de base"),
//!         LocaleString::new("id", "Ini adalah apps biasa"),
//!     ])
//!     .icon(Icon::IconString("pomotimer"))
//!     .try_exec("/usr/local/bin/pomotimer")
//!     .only_show_in(vec![
//!         DesktopEnvironment::HYPRLAND,
//!         DesktopEnvironment::MANGOWM,
//!         DesktopEnvironment::CINNAMON,
//!     ])
//!     .not_show_in(vec![DesktopEnvironment::GNOME])
//!     .exec("/usr/local/bin/some_bin %F")
//!     .mime_types(vec!["text/html", "text/markdown"])
//!     .action(vec![
//!         DesktopAction::new("start-focus", "Start Focus Session")
//!             .icon(Icon::IconPath(Path::new("/home/user/some_icon.jpg")))
//!             .exec("/usr/local/bin/some_bin"),
//!         DesktopAction::new("new-window", "New Window"),
//!         DesktopAction::new(
//!             "localized-action",
//!             vec![
//!                 LocaleString::new("", "Just Test 3"),
//!                 LocaleString::new("id", "Hanya Test 3"),
//!                 LocaleString::new("fr", "il suffit d'en tester 3"),
//!                 LocaleString::new("ja", "テスト3だけ"),
//!             ],
//!         ),
//!     ])
//!     .generic_names(vec![
//!         LocaleString::new("", "Utility"),
//!         LocaleString::new("id", "Utilitas"),
//!         LocaleString::new("fr", "Utilitaire"),
//!         LocaleString::new("ja", "ユーティリティ"),
//!     ])
//!     .keywords(vec![
//!         LocaleStrings::new("", vec!["Utility", "Browser", "Explorer"]),
//!         LocaleStrings::new("id", vec!["Utility", "Browser", "Explorer"]),
//!         LocaleStrings::new("fr", vec!["Utilitaire", "navigateur", "explorateur"]),
//!     ]);
//!
//!     DesktopEntryFile::new(entry).create("my_app.desktop")?;
//!     Ok(())
//! }
//! ```
//!
//! Generated `my_app.desktop`:
//!
//! ```ini
//! [Desktop Entry]
//! Type=Application
//! Name=My App
//! Name[id]=Applikasi ku
//! Name[fr]=mon application
//! Name[ja]=私のアプリ
//! Version=v0.0.1
//! GenericName=Utility
//! GenericName[id]=Utilitas
//! GenericName[fr]=Utilitaire
//! GenericName[ja]=ユーティリティ
//! Comment=This is basic apps
//! Comment[fr]=Ce sont des applications de base
//! Comment[id]=Ini adalah apps biasa
//! Icon=pomotimer
//! OnlyShowIn=Hyprland;Mangowm;Cinnamon;
//! NotShowIn=Gnome;
//! TryExec=/usr/local/bin/pomotimer
//! Exec=/usr/local/bin/some_bin %F
//! MimeType=text/html;text/markdown;
//! Keywords=Utility;Browser;Explorer;
//! Keywords[id]=Utility;Browser;Explorer;
//! Keywords[fr]=Utilitaire;navigateur;explorateur;
//!
//! [Desktop Action start-focus]
//! Name=Start Focus Session
//! Icon=/home/user/some_icon.jpg
//! Exec=/usr/local/bin/some_bin
//!
//! [Desktop Action new-window]
//! Name=New Window
//!
//! [Desktop Action localized-action]
//! Name=Just Test 3
//! Name[id]=Hanya Test 3
//! Name[fr]=il suffit d'en tester 3
//! Name[ja]=テスト3だけ
//! ```

mod desktop_entry;
/// Localization helpers and traits for desktop entry fields.
pub mod locales;
mod utils;

pub use desktop_entry::{
    DesktopAction, DesktopCategories, DesktopEntry, DesktopEntryFile, DesktopEnvironment, Icon,
    LocaleString, LocaleStrings, Types,
};

pub use locales::ToLocale;

pub use DesktopEnvironment::*;
