pub enum BorderStartStartRadius {}
impl crate::Attribute for BorderStartStartRadius {
    const NAME: &'static str = "border-start-start-radius";
}
impl crate::StyleSheet {
    pub fn border_start_start_radius<V: crate::ValueFor<BorderStartStartRadius>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules
            .insert("border-start-start-radius", value.value());
        self
    }
}
