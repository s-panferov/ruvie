pub enum TextRendering {}
impl crate::Attribute for TextRendering {
    const NAME: &'static str = "text-rendering";
}
impl crate::StyleSheet {
    pub fn text_rendering<V: crate::ValueFor<TextRendering>>(mut self, value: V) -> Self {
        self.rules.insert("text-rendering", value.value());
        self
    }
}
