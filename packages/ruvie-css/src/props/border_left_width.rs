pub enum BorderLeftWidth {}
impl crate::Attribute for BorderLeftWidth {
    const NAME: &'static str = "border-left-width";
}
impl crate::StyleSheet {
    pub fn border_left_width<V: crate::ValueFor<BorderLeftWidth>>(mut self, value: V) -> Self {
        self.rules.insert("border-left-width", value.value());
        self
    }
}
