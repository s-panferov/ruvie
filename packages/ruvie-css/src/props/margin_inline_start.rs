pub enum MarginInlineStart {}
impl crate::Attribute for MarginInlineStart {
    const NAME: &'static str = "margin-inline-start";
}
impl crate::StyleSheet {
    pub fn margin_inline_start<V: crate::ValueFor<MarginInlineStart>>(mut self, value: V) -> Self {
        self.rules.insert("margin-inline-start", value.value());
        self
    }
}
