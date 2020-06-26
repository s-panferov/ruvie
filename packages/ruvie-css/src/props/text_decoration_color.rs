pub enum TextDecorationColor {}
impl crate::Attribute for TextDecorationColor {
    const NAME: &'static str = "text-decoration-color";
}
impl crate::StyleSheet {
    pub fn text_decoration_color<V: crate::ValueFor<TextDecorationColor>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("text-decoration-color", value.value());
        self
    }
}

impl crate::ValueFor<TextDecorationColor> for crate::types::color::Color {}
