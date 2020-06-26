pub enum ScrollPaddingTop {
    Auto,
}
impl std::fmt::Display for ScrollPaddingTop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScrollPaddingTop::Auto => write!(f, "auto"),
        }
    }
}
impl crate::ValueFor<ScrollPaddingTop> for ScrollPaddingTop {}
impl crate::Attribute for ScrollPaddingTop {
    const NAME: &'static str = "scroll-padding-top";
}
impl crate::StyleSheet {
    pub fn scroll_padding_top<V: crate::ValueFor<ScrollPaddingTop>>(mut self, value: V) -> Self {
        self.rules.insert("scroll-padding-top", value.value());
        self
    }
}

impl crate::ValueFor<ScrollPaddingTop> for crate::types::length::Length {}
impl crate::ValueFor<ScrollPaddingTop> for crate::types::percentage::Percentage {}
