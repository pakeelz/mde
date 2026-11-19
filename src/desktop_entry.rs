use std::{
    fmt::Display,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use crate::utils;

/// Writes a [`DesktopEntry`] configuration to a `.desktop` file.
pub struct DesktopEntryFile<'a> {
    desktop_entry: DesktopEntry<'a>,
}

impl<'a> DesktopEntryFile<'a> {
    /// Creates a new [`DesktopEntryFile`] wrapping the given [`DesktopEntry`].
    pub fn new(desktop_entry: DesktopEntry<'a>) -> DesktopEntryFile<'a> {
        DesktopEntryFile { desktop_entry }
    }

    /// Writes the formatted `.desktop` content to `output_path`.
    ///
    /// # Errors
    ///
    /// Returns an I/O error if the target file cannot be created, written to, or flushed.
    pub fn create(&self, output_path: impl AsRef<Path>) -> std::io::Result<()> {
        let file = std::fs::File::create(output_path)?;
        let mut writer = BufWriter::new(file);
        writeln!(writer, "[Desktop Entry]")?;
        writeln!(writer, "Type={}", self.desktop_entry.types)?;
        writeln!(writer, "Name={}", self.desktop_entry.name)?;

        if let Some(version) = &self.desktop_entry.version {
            writeln!(writer, "Version={version}")?;
        }

        if let Some(generic_name) = &self.desktop_entry.generic_name {
            writeln!(writer, "GenericName={}", generic_name)?;
        }

        if let Some(no_display) = &self.desktop_entry.no_display {
            writeln!(writer, "NoDisplay={}", no_display)?;
        }

        if let Some(comment) = &self.desktop_entry.comment {
            writeln!(writer, "Comment={}", comment)?;
        }

        if let Some(icon) = &self.desktop_entry.icon {
            writeln!(writer, "Icon={}", icon)?;
        }

        if let Some(hidden) = &self.desktop_entry.hidden {
            writeln!(writer, "Hidden={}", hidden)?;
        }

        if let Some(only_show_in) = &self.desktop_entry.only_show_in {
            utils::write_semicolon_list(&mut writer, "OnlyShowIn", only_show_in)?;
        }

        if let Some(not_show_in) = &self.desktop_entry.not_show_in {
            utils::write_semicolon_list(&mut writer, "NotShowIn", not_show_in)?;
        }

        if let Some(dbus_activatable) = &self.desktop_entry.dbus_activatable {
            writeln!(writer, "DbusActivatable={}", dbus_activatable)?;
        }

        if let Some(try_exec) = &self.desktop_entry.try_exec {
            writeln!(writer, "TryExec={}", try_exec.to_string_lossy())?;
        }

        if let Some(exec) = &self.desktop_entry.exec {
            writeln!(writer, "Exec={}", exec.to_string_lossy())?;
        }

        if let Some(path) = &self.desktop_entry.path {
            writeln!(writer, "Path={}", path.to_string_lossy())?;
        }

        if let Some(terminal) = &self.desktop_entry.terminal {
            writeln!(writer, "Terminal={}", terminal)?;
        }

        if let Some(mime_types) = &self.desktop_entry.mime_types {
            utils::write_semicolon_list(&mut writer, "MimeType", mime_types)?;
        }

        if let Some(categories) = &self.desktop_entry.categories {
            utils::write_semicolon_list(&mut writer, "Categories", categories)?;
        }

        if let Some(interfaces) = &self.desktop_entry.interfaces {
            utils::write_semicolon_list(&mut writer, "Interfaces", interfaces)?;
        }

        if let Some(keywords) = &self.desktop_entry.keywords {
            utils::write_semicolon_list(&mut writer, "Keywords", keywords)?;
        }

        if let Some(startup) = &self.desktop_entry.startup_notify {
            writeln!(writer, "StartupNotify={startup}")?;
        }

        if let Some(wm_class) = &self.desktop_entry.startup_wm_class {
            writeln!(writer, "StartupWMClass={}", wm_class)?;
        }

        if let Some(url) = &self.desktop_entry.url {
            writeln!(writer, "Url={}", url)?;
        }

        if let Some(prefers_non_default) = &self.desktop_entry.prefers_non_default_gpu {
            writeln!(writer, "PrefersNonDefaultGPU={prefers_non_default}")?;
        }

        if let Some(single_main_window) = &self.desktop_entry.single_main_window {
            writeln!(writer, "SingleMainWindow={single_main_window}")?;
        }

        if let Some(action) = &self.desktop_entry.action {
            for action in action {
                writeln!(writer, "\n[Desktop Action {}]", action.action_name)?;
                writeln!(writer, "Name={}", action.name)?;
                if let Some(icon) = &action.icon {
                    writeln!(writer, "Icon={}", icon.to_string_lossy())?;
                }
                if let Some(exec) = &action.exec {
                    writeln!(writer, "Exec={}", exec.to_string_lossy())?;
                }
            }
        }

        writer.flush()?;
        Ok(())
    }
}

/// Builder for Freedesktop Desktop Entry fields.
#[derive(Debug, Default)]
pub struct DesktopEntry<'a> {
    types: Types,
    name: &'a str,
    version: Option<&'a str>,
    generic_name: Option<&'a str>,
    no_display: Option<bool>,
    comment: Option<&'a str>,
    icon: Option<&'a str>,
    hidden: Option<bool>,
    only_show_in: Option<Vec<DesktopEnvironment>>,
    not_show_in: Option<Vec<DesktopEnvironment>>,
    dbus_activatable: Option<bool>,
    try_exec: Option<PathBuf>,
    exec: Option<PathBuf>,
    path: Option<PathBuf>,
    terminal: Option<bool>,
    action: Option<Vec<DesktopAction<'a>>>,
    mime_types: Option<Vec<&'a str>>,
    categories: Option<Vec<DesktopCategories>>,
    interfaces: Option<Vec<&'a str>>,
    keywords: Option<Vec<&'a str>>,
    startup_notify: Option<bool>,
    startup_wm_class: Option<&'a str>,
    url: Option<&'a str>,
    prefers_non_default_gpu: Option<bool>,
    single_main_window: Option<bool>,
}

impl<'a> DesktopEntry<'a> {
    /// Creates a new [`DesktopEntry`] with the required `Name` value.
    pub fn new(name: &'a str) -> DesktopEntry<'a> {
        DesktopEntry {
            name,
            ..Default::default()
        }
    }

    /// Sets the entry `Type` (`Application`, `Link`, or `Directory`).
    pub fn types(mut self, types: Types) -> Self {
        self.types = types;
        self
    }

    /// Sets the `Version` of the desktop entry specification.
    pub fn version(mut self, version: &'a str) -> Self {
        self.version = Some(version);
        self
    }

    /// Sets the `GenericName` for the application.
    pub fn generic_name(mut self, generic_name: &'a str) -> Self {
        self.generic_name = Some(generic_name);
        self
    }

    /// Marks the entry as hidden from menus by setting `NoDisplay` to `true`.
    pub fn no_display(mut self) -> Self {
        self.no_display = Some(true);
        self
    }

    /// Sets the `Comment` describing the entry.
    pub fn comment(mut self, comment: &'a str) -> Self {
        self.comment = Some(comment);
        self
    }

    /// Sets the `Icon` path or icon theme name for this entry.
    pub fn icon(mut self, icon: &'a str) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Marks the entry as ignored by setting `Hidden` to `true`.
    pub fn hidden(mut self) -> Self {
        self.hidden = Some(true);
        self
    }

    /// Restricts the entry to the specified desktop environments via `OnlyShowIn`.
    ///
    /// Use [`DesktopEnvironment::CUSTOM`] if the target environment is not listed.
    pub fn only_show_in(mut self, desktop_environment: Vec<DesktopEnvironment>) -> Self {
        self.only_show_in = Some(desktop_environment);
        self
    }

    /// Excludes the entry from the specified desktop environments via `NotShowIn`.
    ///
    /// Use [`DesktopEnvironment::CUSTOM`] if the target environment is not listed.
    pub fn not_show_in(mut self, desktop_environment: Vec<DesktopEnvironment>) -> Self {
        self.not_show_in = Some(desktop_environment);
        self
    }

    /// Enables D-Bus activation by setting `DBusActivatable` to `true`.
    pub fn dbus_activatable(mut self) -> Self {
        self.dbus_activatable = Some(true);
        self
    }

    /// Sets the `TryExec` path used to test whether the executable exists.
    pub fn try_exec(mut self, path: impl AsRef<Path>) -> Self {
        self.try_exec = Some(path.as_ref().to_path_buf());
        self
    }

    /// Sets the Exec command or executable path for this entry.
    pub fn exec(mut self, path: impl AsRef<Path>) -> Self {
        self.exec = Some(path.as_ref().to_path_buf());
        self
    }

    /// Sets the `Path` working directory for the application when launched.
    pub fn path(mut self, path: impl AsRef<Path>) -> Self {
        self.path = Some(path.as_ref().to_path_buf());
        self
    }

    /// Configures the entry to run in a terminal by setting `Terminal` to `true`.
    pub fn terminal(mut self) -> Self {
        self.terminal = Some(true);
        self
    }

    /// Sets the list of `[Desktop Action ...]` entries associated with this entry.
    pub fn action(mut self, action: Vec<DesktopAction<'a>>) -> Self {
        self.action = Some(action);
        self
    }

    /// Sets the supported `MimeType` list for this entry.
    pub fn mime_types(mut self, mime_type: Vec<&'a str>) -> Self {
        self.mime_types = Some(mime_type);
        self
    }

    /// Sets the `Categories` list for menu classification.
    pub fn categories(mut self, categories: Vec<DesktopCategories>) -> Self {
        self.categories = Some(categories);
        self
    }

    /// Sets the `Interfaces` list implemented by the entry.
    pub fn interfaces(mut self, interfaces: Vec<&'a str>) -> Self {
        self.interfaces = Some(interfaces);
        self
    }

    /// Sets the search `Keywords` for this entry.
    pub fn keywords(mut self, keywords: Vec<&'a str>) -> Self {
        self.keywords = Some(keywords);
        self
    }

    /// Enables startup notification by setting `StartupNotify` to `true`.
    pub fn startup_notify(mut self) -> Self {
        self.startup_notify = Some(true);
        self
    }

    /// Sets the `StartupWMClass` used to match windows with this entry.
    pub fn startup_wm_class(mut self, wm_class: &'a str) -> Self {
        self.startup_wm_class = Some(wm_class);
        self
    }

    /// Sets the `URL` for entries with `Type=Link`.
    pub fn url(mut self, url: &'a str) -> Self {
        self.url = Some(url);
        self
    }

    /// Hints that the application prefers a non-default GPU via `PrefersNonDefaultGPU`.
    pub fn prefers_non_default_gpu(mut self) -> Self {
        self.prefers_non_default_gpu = Some(true);
        self
    }

    /// Hints that the application opens a single main window via `SingleMainWindow`.
    pub fn single_main_window(mut self) -> Self {
        self.single_main_window = Some(true);
        self
    }
}

/// Represents a `[Desktop Action ...]` section within a `.desktop` file.
#[derive(Debug)]
pub struct DesktopAction<'a> {
    action_name: &'a str,
    name: &'a str,
    icon: Option<PathBuf>,
    exec: Option<PathBuf>,
}

impl<'a> DesktopAction<'a> {
    /// Creates a new [`DesktopAction`] with an internal action name and display `Name`.
    pub fn new(action_name: &'a str, name: &'a str) -> DesktopAction<'a> {
        DesktopAction {
            action_name,
            name,
            icon: None,
            exec: None,
        }
    }

    /// Sets the `Icon` path or icon theme name for this action.
    pub fn icon(mut self, icon: impl AsRef<Path>) -> Self {
        self.icon = Some(icon.as_ref().to_path_buf());
        self
    }

    /// Sets the `Exec` command or executable path for this action.
    pub fn exec(mut self, exec: impl AsRef<Path>) -> Self {
        self.exec = Some(exec.as_ref().to_path_buf());
        self
    }
}

/// Target desktop environments or Wayland compositors for `OnlyShowIn` and `NotShowIn`.
#[derive(Debug)]
pub enum DesktopEnvironment {
    GNOME,
    KDE,
    XFCE,
    MATE,
    CINNAMON,
    LXDE,
    LXQT,
    BUDGIE,
    ENLIGHTENMENT,
    PANTHEON,
    DEEPIN,
    HYPRLAND,
    WAYFIRE,
    SWAY,
    LABWC,
    MANGOWM,
    NIRI,
    CUSTOM(String),
}

impl Display for DesktopEnvironment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GNOME => write!(f, "Gnome"),
            Self::KDE => write!(f, "KDE"),
            Self::XFCE => write!(f, "XFCE"),
            Self::MATE => write!(f, "MATE"),
            Self::CINNAMON => write!(f, "Cinnamon"),
            Self::LXDE => write!(f, "LXDE"),
            Self::LXQT => write!(f, "LXQT"),
            Self::BUDGIE => write!(f, "Budgie"),
            Self::ENLIGHTENMENT => write!(f, "Enlightenment"),
            Self::PANTHEON => write!(f, "Pantheon"),
            Self::DEEPIN => write!(f, "Deepin"),
            Self::HYPRLAND => write!(f, "Hyprland"),
            Self::WAYFIRE => write!(f, "Wayfire"),
            Self::SWAY => write!(f, "Sway"),
            Self::LABWC => write!(f, "Labwc"),
            Self::MANGOWM => write!(f, "Mangowm"),
            Self::NIRI => write!(f, "Niri"),
            Self::CUSTOM(x) => {
                let mut chars = x.chars();
                match chars.next() {
                    Some(first) => write!(f, "{}{}", first.to_uppercase(), chars.as_str()),
                    None => Ok(()),
                }
            }
        }
    }
}

/// Menu categories for the `Categories` field in a `.desktop` entry.
#[derive(Debug)]
pub enum DesktopCategories {
    AUDIOVIDEO,
    DEVELOPMENT,
    EDUCATION,
    GRAPHICS,
    NETWORK,
    OFFICE,
    SCIENCE,
    SETTINGS,
    SYSTEM,
    UTILITY,
    CUSTOM(String),
}

impl Display for DesktopCategories {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AUDIOVIDEO => write!(f, "AudioVideo"),
            Self::DEVELOPMENT => write!(f, "Development"),
            Self::EDUCATION => write!(f, "Education"),
            Self::GRAPHICS => write!(f, "Graphics"),
            Self::NETWORK => write!(f, "Network"),
            Self::OFFICE => write!(f, "Office"),
            Self::SCIENCE => write!(f, "Science"),
            Self::SETTINGS => write!(f, "Settings"),
            Self::SYSTEM => write!(f, "System"),
            Self::UTILITY => write!(f, "Utility"),
            Self::CUSTOM(x) => write!(f, "{x}"),
        }
    }
}

/// Desktop entry `Type` values defined by the Freedesktop specification.
#[derive(Debug)]
pub enum Types {
    APPLICATION,
    LINK,
    DIRECTORY,
}

impl Default for Types {
    fn default() -> Self {
        Self::APPLICATION
    }
}

impl Display for Types {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Types::APPLICATION => write!(f, "Application"),
            Types::LINK => write!(f, "Link"),
            Types::DIRECTORY => write!(f, "Directory"),
        }
    }
}
