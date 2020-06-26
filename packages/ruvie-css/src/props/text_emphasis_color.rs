pub enum TextEmphasisColor {}
impl crate::Attribute for TextEmphasisColor {
    const NAME: &'static str = "text-emphasis-color";
}
impl crate::StyleSheet {
    pub fn text_emphasis_color<V: crate::ValueFor<TextEmphasisColor>>(mut self, value: V) -> Self {
        self.rules.insert("text-emphasis-color", value.value());
        self
    }
}

impl crate::ValueFor<TextEmphasisColor> for crate::types::color::Color {}
