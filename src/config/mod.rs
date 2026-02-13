mod anchor;
mod entry;
mod font;
mod namespace;

use anyhow::{Context, Result, bail};
use serde::Deserialize;
use smart_default::SmartDefault;
use std::collections::HashMap;
use std::str::FromStr;

pub use self::anchor::ConfigAnchor;
pub use self::entry::Entry;
pub use self::font::Font;
pub use self::namespace::Namespace;
use crate::color::Color;
use crate::key::Key;

#[derive(Deserialize, SmartDefault)]
#[serde(deny_unknown_fields, default)]
pub struct Colors {
    #[default(Color::from_rgba_hex(0x282828ff))]
    pub background: Color,
    #[default(Color::from_rgba_hex(0xfbf1c7ff))]
    pub foreground: Color,
    #[default(Color::from_rgba_hex(0x8ec07cff))]
    pub border: Color,
    #[default(Color::from_rgba_hex(0xebdbb2ff))]
    pub accent: Color,
}

#[derive(Deserialize, SmartDefault)]
#[serde(deny_unknown_fields, default)]
pub struct Theme {
    pub colors: Colors,

    pub margin_top: i32,
    pub margin_right: i32,
    pub margin_bottom: i32,
    pub margin_left: i32,

    pub font: Font,

    #[default(" ➜ ".into())]
    pub separator: String,

    #[default("+".into())]
    pub submenu_indicator: String,

    #[default(4.0)]
    pub border_width: f64,

    #[default(20.0)]
    pub corner_radius: f64,

    pub padding: Option<f64>,
    pub rows_per_column: Option<usize>,
    pub column_padding: Option<f64>,
}

#[derive(Deserialize, SmartDefault)]
#[serde(deny_unknown_fields, default)]
pub struct App {
    pub anchor: ConfigAnchor,

    #[default(Some(crate::key::SingleKey::from_str("Escape").unwrap().into()))]
    pub quit_key: Option<Key>,

    #[default(Namespace::new(c"wlr_which_key".to_owned()))]
    pub namespace: Namespace,

    pub inhibit_compositor_keyboard_shortcuts: bool,
    pub auto_kbd_layout: bool,
}

#[derive(Deserialize, SmartDefault)]
#[serde(deny_unknown_fields, default)]
pub struct Config {
    pub app: App,
    pub theme: Theme,
    pub menu: HashMap<String, Vec<Entry>>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let mut config_path = dirs::config_dir().context("Could not find config directory")?;
        config_path.push("wlr-which-key");
        config_path.push(if cfg!(debug_assertions) {
            "config-test"
        } else {
            "config"
        });
        config_path.set_extension("toml");

        if !config_path.exists() {
            bail!("config file not found: {}", config_path.display());
        }

        let config_str =
            std::fs::read_to_string(config_path).context("Failed to read configuration")?;

        toml::from_str(&config_str).context("Failed to deserialize configuration")
    }

    pub fn padding(&self) -> f64 {
        self.theme.padding.unwrap_or(self.theme.corner_radius)
    }

    pub fn column_padding(&self) -> f64 {
        self.theme.column_padding.unwrap_or_else(|| self.padding())
    }
}
