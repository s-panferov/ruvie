pub enum ScrollPaddingLeft {
    Auto,
}
impl std::fmt::Display for ScrollPaddingLeft {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScrollPaddingLeft::Auto => write!(f, "auto"),
        }
    }
}
impl crate::ValueFor<ScrollPaddingLeft> for ScrollPaddingLeft {}
impl crate::Attribute for ScrollPaddingLeft {
    const NAME: &'static str = "scroll-padding-left";
}
impl crate::StyleSheet {
    pub fn scroll_padding_left<V: crate::ValueFor<ScrollPaddingLeft>>(mut self, value: V) -> Self {
        self.rules.insert("scroll-padding-left", value.value());
        self
    }
}

impl crate::ValueFor<ScrollPaddingLeft> for crate::types::length::Length {}
impl crate::ValueFor<ScrollPaddingLeft> for crate::types::percentage::Percentage {}
