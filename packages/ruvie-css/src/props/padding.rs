pub enum Padding {}
impl crate::Attribute for Padding {
    const NAME: &'static str = "padding";
}
impl crate::StyleSheet {
    pub fn padding<V: crate::ValueFor<Padding>>(mut self, value: V) -> Self {
        self.rules.insert("padding", value.value());
        self
    }
}
