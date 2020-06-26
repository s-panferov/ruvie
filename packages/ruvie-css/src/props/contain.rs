pub enum Contain {}
impl crate::Attribute for Contain {
    const NAME: &'static str = "contain";
}
impl crate::StyleSheet {
    pub fn contain<V: crate::ValueFor<Contain>>(mut self, value: V) -> Self {
        self.rules.insert("contain", value.value());
        self
    }
}
