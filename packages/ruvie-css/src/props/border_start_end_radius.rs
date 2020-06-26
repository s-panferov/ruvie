pub enum BorderStartEndRadiu {}
impl crate::Attribute for BorderStartEndRadiu {
    const NAME: &'static str = "border-start-end-radius";
}
impl crate::StyleSheet {
    pub fn border_start_end_radius<V: crate::ValueFor<BorderStartEndRadiu>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("border-start-end-radius", value.value());
        self
    }
}
