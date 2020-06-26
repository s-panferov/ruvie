pub enum Margin {}
impl crate::Attribute for Margin {
    const NAME: &'static str = "margin";
}
impl crate::StyleSheet {
    pub fn margin<V: crate::ValueFor<Margin>>(mut self, value: V) -> Self {
        self.rules.insert("margin", value.value());
        self
    }
}
