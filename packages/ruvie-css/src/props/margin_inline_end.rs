pub enum MarginInlineEnd {}
impl crate::Attribute for MarginInlineEnd {
    const NAME: &'static str = "margin-inline-end";
}
impl crate::StyleSheet {
    pub fn margin_inline_end<V: crate::ValueFor<MarginInlineEnd>>(mut self, value: V) -> Self {
        self.rules.insert("margin-inline-end", value.value());
        self
    }
}
