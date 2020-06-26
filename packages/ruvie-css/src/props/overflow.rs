pub enum Overflow {}
impl crate::Attribute for Overflow {
    const NAME: &'static str = "overflow";
}
impl crate::StyleSheet {
    pub fn overflow<V: crate::ValueFor<Overflow>>(mut self, value: V) -> Self {
        self.rules.insert("overflow", value.value());
        self
    }
}
