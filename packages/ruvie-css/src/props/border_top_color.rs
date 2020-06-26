pub enum BorderTopColor {}
impl crate::Attribute for BorderTopColor {
    const NAME: &'static str = "border-top-color";
}
impl crate::StyleSheet {
    pub fn border_top_color<V: crate::ValueFor<BorderTopColor>>(mut self, value: V) -> Self {
        self.rules.insert("border-top-color", value.value());
        self
    }
}

impl crate::ValueFor<BorderTopColor> for crate::types::color::Color {}
