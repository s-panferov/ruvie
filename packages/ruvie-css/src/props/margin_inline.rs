pub enum MarginInline {}
impl crate::Attribute for MarginInline {
    const NAME: &'static str = "margin-inline";
}
impl crate::StyleSheet {
    pub fn margin_inline<V: crate::ValueFor<MarginInline>>(mut self, value: V) -> Self {
        self.rules.insert("margin-inline", value.value());
        self
    }
}
