use pangocairo::pango;
use pangocairo::pango::FontDescription;

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Font {
    pub family: String,
    pub size: i32,
}

impl Font {
    pub fn as_font_desc(&self) -> FontDescription {
        let mut desc = FontDescription::new();
        desc.set_family(&self.family);
        desc.set_size(self.size * pango::SCALE);
        desc
    }
}

impl Default for Font {
    fn default() -> Self {
        Self {
            family: "monospace".into(),
            size: 10,
        }
    }
}
