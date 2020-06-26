pub enum BorderRadius {}
impl crate::Attribute for BorderRadius {
    const NAME: &'static str = "border-radius";
}
impl crate::StyleSheet {
    pub fn border_radius<V: crate::ValueFor<BorderRadius>>(mut self, value: V) -> Self {
        self.rules.insert("border-radius", value.value());
        self
    }
}
