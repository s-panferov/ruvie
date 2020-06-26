pub enum BorderStartEndRadius {}
impl crate::Attribute for BorderStartEndRadius {
    const NAME: &'static str = "border-start-end-radius";
}
impl crate::StyleSheet {
    pub fn border_start_end_radius<V: crate::ValueFor<BorderStartEndRadius>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("border-start-end-radius", value.value());
        self
    }
}
