pub enum Border {}
impl crate::Attribute for Border {
    const NAME: &'static str = "border";
}
impl crate::StyleSheet {
    pub fn border<V: crate::ValueFor<Border>>(mut self, value: V) -> Self {
        self.rules.insert("border", value.value());
        self
    }
}
