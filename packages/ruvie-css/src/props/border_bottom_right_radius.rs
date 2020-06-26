pub enum BorderBottomRightRadiu {}
impl crate::Attribute for BorderBottomRightRadiu {
    const NAME: &'static str = "border-bottom-right-radius";
}
impl crate::StyleSheet {
    pub fn border_bottom_right_radius<V: crate::ValueFor<BorderBottomRightRadiu>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules
            .insert("border-bottom-right-radius", value.value());
        self
    }
}
