pub enum ScrollPaddingBlockEnd {
    Auto,
}
impl std::fmt::Display for ScrollPaddingBlockEnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScrollPaddingBlockEnd::Auto => write!(f, "auto"),
        }
    }
}
impl crate::ValueFor<ScrollPaddingBlockEnd> for ScrollPaddingBlockEnd {}
impl crate::Attribute for ScrollPaddingBlockEnd {
    const NAME: &'static str = "scroll-padding-block-end";
}
impl crate::StyleSheet {
    pub fn scroll_padding_block_end<V: crate::ValueFor<ScrollPaddingBlockEnd>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("scroll-padding-block-end", value.value());
        self
    }
}

impl crate::ValueFor<ScrollPaddingBlockEnd> for crate::types::length::Length {}
impl crate::ValueFor<ScrollPaddingBlockEnd> for crate::types::percentage::Percentage {}
