pub enum BorderTopWidth {}
impl crate::Attribute for BorderTopWidth {
    const NAME: &'static str = "border-top-width";
}
impl crate::StyleSheet {
    pub fn border_top_width<V: crate::ValueFor<BorderTopWidth>>(mut self, value: V) -> Self {
        self.rules.insert("border-top-width", value.value());
        self
    }
}
