# mde

A Rust library for building and writing Freedesktop Desktop Entry (`.desktop`) files on Linux.

## Features

- Builder API via [`DesktopEntry`] and [`DesktopAction`] for configuring entry fields and desktop actions.
- Typed enums for entry [`Types`], menu [`DesktopCategories`], and [`DesktopEnvironment`] targets (including Wayland compositors and custom values).
- Buffered file output via [`DesktopEntryFile`].

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mde = "0.1.0"
```

or run:
```bash
cargo add mde
```

## Usage
```rust
use mde::{DesktopAction, DesktopEntry, DesktopEntryFile, DesktopEnvironment, Types};

fn main() -> std::io::Result<()> {
    let entry = DesktopEntry::new("Pomotimer")
        .types(Types::APPLICATION)
        .version("v0.0.1")
        .comment("Focus timer for Linux desktop sessions")
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
                .icon("/home/user/some_icon.jpg")
                .exec("/usr/local/bin/pomotimer --focus"),
            DesktopAction::new("new-window", "New Window"),
        ]);

    DesktopEntryFile::new(entry).create("my_app.desktop")?;
    Ok(())
}
```
