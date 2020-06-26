pub enum BorderLeftStyle {}
impl crate::Attribute for BorderLeftStyle {
    const NAME: &'static str = "border-left-style";
}
impl crate::StyleSheet {
    pub fn border_left_style<V: crate::ValueFor<BorderLeftStyle>>(mut self, value: V) -> Self {
        self.rules.insert("border-left-style", value.value());
        self
    }
}
