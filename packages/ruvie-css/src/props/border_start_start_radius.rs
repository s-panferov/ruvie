pub enum BorderStartStartRadiu {}
impl crate::Attribute for BorderStartStartRadiu {
    const NAME: &'static str = "border-start-start-radius";
}
impl crate::StyleSheet {
    pub fn border_start_start_radius<V: crate::ValueFor<BorderStartStartRadiu>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules
            .insert("border-start-start-radius", value.value());
        self
    }
}
