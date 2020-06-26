pub enum ScrollPadding {}
impl crate::Attribute for ScrollPadding {
    const NAME: &'static str = "scroll-padding";
}
impl crate::StyleSheet {
    pub fn scroll_padding<V: crate::ValueFor<ScrollPadding>>(mut self, value: V) -> Self {
        self.rules.insert("scroll-padding", value.value());
        self
    }
}
