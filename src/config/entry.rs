use serde::Deserialize;

use crate::key::Key;

#[derive(Deserialize)]
#[serde(deny_unknown_fields, untagged)]
pub enum Entry {
    Cmd {
        key: Key,
        desc: String,
        cmd: String,

        #[serde(default)]
        keep_open: bool,
    },
    Recursive {
        key: Key,
        desc: String,
        submenu: String,
    },
}
