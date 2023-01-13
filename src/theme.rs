
#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    pub foreground: String,
    pub background: String,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
          foreground: "black".to_owned(),
          background: "white".to_owned(),
        }
    }
}
