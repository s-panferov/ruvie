pub enum TextShadow {}
impl crate::Attribute for TextShadow {
    const NAME: &'static str = "text-shadow";
}
impl crate::StyleSheet {
    pub fn text_shadow<V: crate::ValueFor<TextShadow>>(mut self, value: V) -> Self {
        self.rules.insert("text-shadow", value.value());
        self
    }
}
