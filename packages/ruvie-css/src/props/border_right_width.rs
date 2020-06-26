pub enum BorderRightWidth {}
impl crate::Attribute for BorderRightWidth {
    const NAME: &'static str = "border-right-width";
}
impl crate::StyleSheet {
    pub fn border_right_width<V: crate::ValueFor<BorderRightWidth>>(mut self, value: V) -> Self {
        self.rules.insert("border-right-width", value.value());
        self
    }
}
