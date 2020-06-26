pub enum BorderEndEndRadius {}
impl crate::Attribute for BorderEndEndRadius {
    const NAME: &'static str = "border-end-end-radius";
}
impl crate::StyleSheet {
    pub fn border_end_end_radius<V: crate::ValueFor<BorderEndEndRadius>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("border-end-end-radius", value.value());
        self
    }
}
