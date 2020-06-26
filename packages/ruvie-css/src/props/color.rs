pub enum Color {}
impl crate::Attribute for Color {
    const NAME: &'static str = "color";
}
impl crate::StyleSheet {
    pub fn color<V: crate::ValueFor<Color>>(mut self, value: V) -> Self {
        self.rules.insert("color", value.value());
        self
    }
}

impl crate::ValueFor<Color> for crate::types::color::Color {}
