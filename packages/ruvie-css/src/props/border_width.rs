pub enum BorderWidth {}
impl crate::Attribute for BorderWidth {
    const NAME: &'static str = "border-width";
}
impl crate::StyleSheet {
    pub fn border_width<V: crate::ValueFor<BorderWidth>>(mut self, value: V) -> Self {
        self.rules.insert("border-width", value.value());
        self
    }
}
