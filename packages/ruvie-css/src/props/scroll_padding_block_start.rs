pub enum ScrollPaddingBlockStart {
    Auto,
}
impl std::fmt::Display for ScrollPaddingBlockStart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScrollPaddingBlockStart::Auto => write!(f, "auto"),
        }
    }
}
impl crate::ValueFor<ScrollPaddingBlockStart> for ScrollPaddingBlockStart {}
impl crate::Attribute for ScrollPaddingBlockStart {
    const NAME: &'static str = "scroll-padding-block-start";
}
impl crate::StyleSheet {
    pub fn scroll_padding_block_start<V: crate::ValueFor<ScrollPaddingBlockStart>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules
            .insert("scroll-padding-block-start", value.value());
        self
    }
}

impl crate::ValueFor<ScrollPaddingBlockStart> for crate::types::length::Length {}
impl crate::ValueFor<ScrollPaddingBlockStart> for crate::types::percentage::Percentage {}
