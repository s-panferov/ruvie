pub enum ScrollPaddingBottom {
    Auto,
}
impl std::fmt::Display for ScrollPaddingBottom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScrollPaddingBottom::Auto => write!(f, "auto"),
        }
    }
}
impl crate::ValueFor<ScrollPaddingBottom> for ScrollPaddingBottom {}
impl crate::Attribute for ScrollPaddingBottom {
    const NAME: &'static str = "scroll-padding-bottom";
}
impl crate::StyleSheet {
    pub fn scroll_padding_bottom<V: crate::ValueFor<ScrollPaddingBottom>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("scroll-padding-bottom", value.value());
        self
    }
}

impl crate::ValueFor<ScrollPaddingBottom> for crate::types::length::Length {}
impl crate::ValueFor<ScrollPaddingBottom> for crate::types::percentage::Percentage {}
