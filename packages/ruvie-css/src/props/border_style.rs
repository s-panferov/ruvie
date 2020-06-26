pub enum BorderStyle {}
impl crate::Attribute for BorderStyle {
    const NAME: &'static str = "border-style";
}
impl crate::StyleSheet {
    pub fn border_style<V: crate::ValueFor<BorderStyle>>(mut self, value: V) -> Self {
        self.rules.insert("border-style", value.value());
        self
    }
}
