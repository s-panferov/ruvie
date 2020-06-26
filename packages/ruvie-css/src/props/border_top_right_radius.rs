pub enum BorderTopRightRadius {}
impl crate::Attribute for BorderTopRightRadius {
    const NAME: &'static str = "border-top-right-radius";
}
impl crate::StyleSheet {
    pub fn border_top_right_radius<V: crate::ValueFor<BorderTopRightRadius>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("border-top-right-radius", value.value());
        self
    }
}
