pub enum BorderRight {}
impl crate::Attribute for BorderRight {
    const NAME: &'static str = "border-right";
}
impl crate::StyleSheet {
    pub fn border_right<V: crate::ValueFor<BorderRight>>(mut self, value: V) -> Self {
        self.rules.insert("border-right", value.value());
        self
    }
}
