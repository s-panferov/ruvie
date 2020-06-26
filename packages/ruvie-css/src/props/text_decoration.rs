pub enum TextDecoration {}
impl crate::Attribute for TextDecoration {
    const NAME: &'static str = "text-decoration";
}
impl crate::StyleSheet {
    pub fn text_decoration<V: crate::ValueFor<TextDecoration>>(mut self, value: V) -> Self {
        self.rules.insert("text-decoration", value.value());
        self
    }
}
