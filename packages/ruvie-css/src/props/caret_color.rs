pub enum CaretColor {
    Auto,
}
impl std::fmt::Display for CaretColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CaretColor::Auto => write!(f, "auto"),
        }
    }
}
impl crate::ValueFor<CaretColor> for CaretColor {}
impl crate::Attribute for CaretColor {
    const NAME: &'static str = "caret-color";
}
impl crate::StyleSheet {
    pub fn caret_color<V: crate::ValueFor<CaretColor>>(mut self, value: V) -> Self {
        self.rules.insert("caret-color", value.value());
        self
    }
}

impl crate::ValueFor<CaretColor> for crate::types::color::Color {}
