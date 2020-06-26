pub enum ScrollMarginBottom {}
impl crate::Attribute for ScrollMarginBottom {
    const NAME: &'static str = "scroll-margin-bottom";
}
impl crate::StyleSheet {
    pub fn scroll_margin_bottom<V: crate::ValueFor<ScrollMarginBottom>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("scroll-margin-bottom", value.value());
        self
    }
}
impl crate::ValueFor<ScrollMarginBottom> for crate::types::length::Length {}
