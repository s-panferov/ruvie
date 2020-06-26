pub enum ScrollMargin {}
impl crate::Attribute for ScrollMargin {
    const NAME: &'static str = "scroll-margin";
}
impl crate::StyleSheet {
    pub fn scroll_margin<V: crate::ValueFor<ScrollMargin>>(mut self, value: V) -> Self {
        self.rules.insert("scroll-margin", value.value());
        self
    }
}
