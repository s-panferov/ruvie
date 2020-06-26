pub enum ScrollPaddingRight {
    Auto,
}
impl std::fmt::Display for ScrollPaddingRight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScrollPaddingRight::Auto => write!(f, "auto"),
        }
    }
}
impl crate::ValueFor<ScrollPaddingRight> for ScrollPaddingRight {}
impl crate::Attribute for ScrollPaddingRight {
    const NAME: &'static str = "scroll-padding-right";
}
impl crate::StyleSheet {
    pub fn scroll_padding_right<V: crate::ValueFor<ScrollPaddingRight>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("scroll-padding-right", value.value());
        self
    }
}

impl crate::ValueFor<ScrollPaddingRight> for crate::types::length::Length {}
impl crate::ValueFor<ScrollPaddingRight> for crate::types::percentage::Percentage {}
