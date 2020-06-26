pub enum BorderLeftColor {}
impl crate::Attribute for BorderLeftColor {
    const NAME: &'static str = "border-left-color";
}
impl crate::StyleSheet {
    pub fn border_left_color<V: crate::ValueFor<BorderLeftColor>>(mut self, value: V) -> Self {
        self.rules.insert("border-left-color", value.value());
        self
    }
}

impl crate::ValueFor<BorderLeftColor> for crate::types::color::Color {}
