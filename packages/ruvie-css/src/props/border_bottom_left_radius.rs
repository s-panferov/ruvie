pub enum BorderBottomLeftRadiu {}
impl crate::Attribute for BorderBottomLeftRadiu {
    const NAME: &'static str = "border-bottom-left-radius";
}
impl crate::StyleSheet {
    pub fn border_bottom_left_radius<V: crate::ValueFor<BorderBottomLeftRadiu>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules
            .insert("border-bottom-left-radius", value.value());
        self
    }
}
