//! Builder and writer for Freedesktop Desktop Entry (`.desktop`) files on Linux.
//!
//! This crate provides a builder API for defining desktop entry metadata,
//! desktop actions, target desktop environments, and categories, then writing
//! the resulting `.desktop` file to disk.
//!
//! # Examples
//!
//! ```no_run
//! use mde::{DesktopAction, DesktopEntry, DesktopEntryFile, DesktopEnvironment, Types};
//!
//! fn main() -> std::io::Result<()> {
//!     let entry = DesktopEntry::new("Pomotimer")
//!         .types(Types::APPLICATION)
//!         .version("v0.0.1")
//!         .comment("Focus timer for Linux desktop sessions")
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
//!                 .icon("/home/user/some_icon.jpg")
//!                 .exec("/usr/local/bin/pomotimer --focus"),
//!             DesktopAction::new("new-window", "New Window"),
//!         ]);
//!
//!     DesktopEntryFile::new(entry).create("my_apps.desktop")?;
//!     Ok(())
//! }
//! ```

mod desktop_entry;
mod utils;

pub use desktop_entry::{
    DesktopAction, DesktopCategories, DesktopEntry, DesktopEntryFile, DesktopEnvironment, Types,
};

pub use DesktopEnvironment::*;
