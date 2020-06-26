pub enum BorderEndEndRadiu {}
impl crate::Attribute for BorderEndEndRadiu {
    const NAME: &'static str = "border-end-end-radius";
}
impl crate::StyleSheet {
    pub fn border_end_end_radius<V: crate::ValueFor<BorderEndEndRadiu>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("border-end-end-radius", value.value());
        self
    }
}
