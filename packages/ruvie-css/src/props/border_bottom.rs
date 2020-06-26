pub enum BorderBottom {}
impl crate::Attribute for BorderBottom {
    const NAME: &'static str = "border-bottom";
}
impl crate::StyleSheet {
    pub fn border_bottom<V: crate::ValueFor<BorderBottom>>(mut self, value: V) -> Self {
        self.rules.insert("border-bottom", value.value());
        self
    }
}
