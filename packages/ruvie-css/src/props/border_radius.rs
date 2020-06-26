pub enum BorderRadiu {}
impl crate::Attribute for BorderRadiu {
    const NAME: &'static str = "border-radius";
}
impl crate::StyleSheet {
    pub fn border_radius<V: crate::ValueFor<BorderRadiu>>(mut self, value: V) -> Self {
        self.rules.insert("border-radius", value.value());
        self
    }
}
