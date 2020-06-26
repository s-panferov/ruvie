pub enum BorderRightStyle {}
impl crate::Attribute for BorderRightStyle {
    const NAME: &'static str = "border-right-style";
}
impl crate::StyleSheet {
    pub fn border_right_style<V: crate::ValueFor<BorderRightStyle>>(mut self, value: V) -> Self {
        self.rules.insert("border-right-style", value.value());
        self
    }
}
