pub enum ScrollMarginTop {}
impl crate::Attribute for ScrollMarginTop {
    const NAME: &'static str = "scroll-margin-top";
}
impl crate::StyleSheet {
    pub fn scroll_margin_top<V: crate::ValueFor<ScrollMarginTop>>(mut self, value: V) -> Self {
        self.rules.insert("scroll-margin-top", value.value());
        self
    }
}
impl crate::ValueFor<ScrollMarginTop> for crate::types::length::Length {}
