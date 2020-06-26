pub enum BorderTopStyle {}
impl crate::Attribute for BorderTopStyle {
    const NAME: &'static str = "border-top-style";
}
impl crate::StyleSheet {
    pub fn border_top_style<V: crate::ValueFor<BorderTopStyle>>(mut self, value: V) -> Self {
        self.rules.insert("border-top-style", value.value());
        self
    }
}
