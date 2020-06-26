pub enum OutlineColor {
    Invert,
}
impl std::fmt::Display for OutlineColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutlineColor::Invert => write!(f, "invert"),
        }
    }
}
impl crate::ValueFor<OutlineColor> for OutlineColor {}
impl crate::Attribute for OutlineColor {
    const NAME: &'static str = "outline-color";
}
impl crate::StyleSheet {
    pub fn outline_color<V: crate::ValueFor<OutlineColor>>(mut self, value: V) -> Self {
        self.rules.insert("outline-color", value.value());
        self
    }
}

impl crate::ValueFor<OutlineColor> for crate::types::color::Color {}
