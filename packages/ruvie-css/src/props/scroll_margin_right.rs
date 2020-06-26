pub enum ScrollMarginRight {}
impl crate::Attribute for ScrollMarginRight {
    const NAME: &'static str = "scroll-margin-right";
}
impl crate::StyleSheet {
    pub fn scroll_margin_right<V: crate::ValueFor<ScrollMarginRight>>(mut self, value: V) -> Self {
        self.rules.insert("scroll-margin-right", value.value());
        self
    }
}
impl crate::ValueFor<ScrollMarginRight> for crate::types::length::Length {}
