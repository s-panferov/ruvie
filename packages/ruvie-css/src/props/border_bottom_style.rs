pub enum BorderBottomStyle {}
impl crate::Attribute for BorderBottomStyle {
    const NAME: &'static str = "border-bottom-style";
}
impl crate::StyleSheet {
    pub fn border_bottom_style<V: crate::ValueFor<BorderBottomStyle>>(mut self, value: V) -> Self {
        self.rules.insert("border-bottom-style", value.value());
        self
    }
}
