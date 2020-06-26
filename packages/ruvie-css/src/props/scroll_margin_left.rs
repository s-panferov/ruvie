pub enum ScrollMarginLeft {}
impl crate::Attribute for ScrollMarginLeft {
    const NAME: &'static str = "scroll-margin-left";
}
impl crate::StyleSheet {
    pub fn scroll_margin_left<V: crate::ValueFor<ScrollMarginLeft>>(mut self, value: V) -> Self {
        self.rules.insert("scroll-margin-left", value.value());
        self
    }
}
impl crate::ValueFor<ScrollMarginLeft> for crate::types::length::Length {}
