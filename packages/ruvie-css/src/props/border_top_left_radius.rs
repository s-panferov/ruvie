pub enum BorderTopLeftRadius {}
impl crate::Attribute for BorderTopLeftRadius {
    const NAME: &'static str = "border-top-left-radius";
}
impl crate::StyleSheet {
    pub fn border_top_left_radius<V: crate::ValueFor<BorderTopLeftRadius>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("border-top-left-radius", value.value());
        self
    }
}
