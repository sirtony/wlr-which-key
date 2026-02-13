# wlr-which-key

Keymap manager for wlroots-based compositors. Inspired by [which-key.nvim](https://github.com/folke/which-key.nvim).

## Installation

### From Source

```sh
git clone --depth 1 https://github.com/sirtony/wlr-which-key.git
cd wlr-which-key
cargo install --path . --locked
```

## Usage

```sh
wlr-which-key [config_name]                    # Start with default menu
wlr-which-key --initial-keys "p s"             # Navigate to submenu or execute command
```

## Configuration

Default config file: `$XDG_CONFIG_HOME/wlr-which-key/config.yaml` or `~/.config/wlr-which-key/config.yaml`. Run `wlr-which-key --help` for more info.

Keybindings may be single characters (e.g. `a`, `B`) or [xkb key labels](https://github.com/xkbcommon/libxkbcommon/blob/master/include/xkbcommon/xkbcommon-keysyms.h) (without the `XKB_KEY_` prefix, e.g. `Return`, `Insert`). Ctrl, Alt, and Mod4/Logo modifiers are supported (like `Ctrl+Return` or `Ctrl+Alt+a` or `Mod4+Return` or `Logo+Return`). A `key` may also be a list of strings, in which case a keybinding will match if any of the keys match (e.g. `key: [Left, h]`) will match both left arrow and 'h'.

When executed a command will normally end the `wlr_which_key` process. If you want certain commands to keep the UI open after they execute then
configure those specific commands with (`keep_open: true`).

Example config:

```toml
# Theming
[theme]
font = "JetBrainsMono Nerd Font 12"
separator = " \uf444 "
border_width = 2
corner_radius = 10
padding = 15 # Defaults to corner_r
rows_per_column = 5 # No limit by default
column_padding = 25 # Defaults to padding
margin_right = 0
margin_bottom = 0
margin_left = 0
margin_top = 0

[theme.colors]
background = "#282828d0"
foreground = "#fbf1c7"
border = "#8ec07c"

[app]
anchor = "center" # One of center, left, right, top, bottom, bottom-left, top-left, etc.
namespace = "wlr_which_key" # namespace to use for the layer shell surface

# Permits key bindings that conflict with compositor key bindings.
# Default is `false`.
inhibit_compositor_keyboard_shortcuts = true

# Try to guess the correct keyboard layout to use. Default is `false`.
auto_kbd_layout = true

[[menu]]
key = "p"
desc = "Power"

[[menu.submenu]]
key = "s"
desc = "Sleep"
cmd = "systemctl suspend"

[[menu.submenu]]
key = "r"
desc = "Reboot"
cmd = "reboot"

[[menu.submenu]]
key = "o"
desc = "Off"
cmd = "poweroff"


[[menu]]
key = "l"
desc = "Laptop Screen"

[[menu.submenu]]
key = "t"
desc = "Toggle On/Off"
cmd = "toggle-laptop-display.sh"

[[menu.submenu]]
key = "s"
desc = "Scale"

[[menu.submenu.submenu]]
key = "1"
desc = "Set Scale to 1.0"
cmd = "wlr-randr --output eDP-1 --scale 1"

[[menu.submenu.submenu]]
key = "2"
desc = "Set Scale to 1.1"
cmd = "wlr-randr --output eDP-1 --scale 1.1"

[[menu.submenu.submenu]]
key = "3"
desc = "Set Scale to 1.2"
cmd = "wlr-randr --output eDP-1 --scale 1.2"

[[menu.submenu.submenu]]
key = "4"
desc = "Set Scale to 1.3"
cmd = "wlr-randr --output eDP-1 --scale 1.3"
```

![image](https://user-images.githubusercontent.com/34583604/233025292-af0d5798-1854-4809-b08f-2e8f1a65b3ce.png)

![image](https://user-images.githubusercontent.com/34583604/233025368-e59a386a-6a52-4168-a6e3-5102ea6329cf.png)
