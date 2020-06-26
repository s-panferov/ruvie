pub enum ScrollPaddingInlineStart {
    Auto,
}
impl std::fmt::Display for ScrollPaddingInlineStart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScrollPaddingInlineStart::Auto => write!(f, "auto"),
        }
    }
}
impl crate::ValueFor<ScrollPaddingInlineStart> for ScrollPaddingInlineStart {}
impl crate::Attribute for ScrollPaddingInlineStart {
    const NAME: &'static str = "scroll-padding-inline-start";
}
impl crate::StyleSheet {
    pub fn scroll_padding_inline_start<V: crate::ValueFor<ScrollPaddingInlineStart>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules
            .insert("scroll-padding-inline-start", value.value());
        self
    }
}

impl crate::ValueFor<ScrollPaddingInlineStart> for crate::types::length::Length {}
impl crate::ValueFor<ScrollPaddingInlineStart> for crate::types::percentage::Percentage {}
