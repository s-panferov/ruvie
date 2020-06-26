pub enum ScrollPaddingInlineEnd {
    Auto,
}
impl std::fmt::Display for ScrollPaddingInlineEnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScrollPaddingInlineEnd::Auto => write!(f, "auto"),
        }
    }
}
impl crate::ValueFor<ScrollPaddingInlineEnd> for ScrollPaddingInlineEnd {}
impl crate::Attribute for ScrollPaddingInlineEnd {
    const NAME: &'static str = "scroll-padding-inline-end";
}
impl crate::StyleSheet {
    pub fn scroll_padding_inline_end<V: crate::ValueFor<ScrollPaddingInlineEnd>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules
            .insert("scroll-padding-inline-end", value.value());
        self
    }
}

impl crate::ValueFor<ScrollPaddingInlineEnd> for crate::types::length::Length {}
impl crate::ValueFor<ScrollPaddingInlineEnd> for crate::types::percentage::Percentage {}
