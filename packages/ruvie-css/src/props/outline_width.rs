pub enum OutlineWidth {}
impl crate::Attribute for OutlineWidth {
    const NAME: &'static str = "outline-width";
}
impl crate::StyleSheet {
    pub fn outline_width<V: crate::ValueFor<OutlineWidth>>(mut self, value: V) -> Self {
        self.rules.insert("outline-width", value.value());
        self
    }
}
