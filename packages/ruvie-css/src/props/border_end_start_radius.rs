pub enum BorderEndStartRadiu {}
impl crate::Attribute for BorderEndStartRadiu {
    const NAME: &'static str = "border-end-start-radius";
}
impl crate::StyleSheet {
    pub fn border_end_start_radius<V: crate::ValueFor<BorderEndStartRadiu>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("border-end-start-radius", value.value());
        self
    }
}
