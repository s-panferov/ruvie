pub enum BorderBottomWidth {}
impl crate::Attribute for BorderBottomWidth {
    const NAME: &'static str = "border-bottom-width";
}
impl crate::StyleSheet {
    pub fn border_bottom_width<V: crate::ValueFor<BorderBottomWidth>>(mut self, value: V) -> Self {
        self.rules.insert("border-bottom-width", value.value());
        self
    }
}
