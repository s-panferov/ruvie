pub enum OverflowInline {
    Auto,
    Clip,
    Hidden,
    Scroll,
    Visible,
}
impl std::fmt::Display for OverflowInline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OverflowInline::Auto => write!(f, "auto"),
            OverflowInline::Clip => write!(f, "clip"),
            OverflowInline::Hidden => write!(f, "hidden"),
            OverflowInline::Scroll => write!(f, "scroll"),
            OverflowInline::Visible => write!(f, "visible"),
        }
    }
}
impl crate::ValueFor<OverflowInline> for OverflowInline {}
impl crate::Attribute for OverflowInline {
    const NAME: &'static str = "overflow-inline";
}
impl crate::StyleSheet {
    pub fn overflow_inline<V: crate::ValueFor<OverflowInline>>(mut self, value: V) -> Self {
        self.rules.insert("overflow-inline", value.value());
        self
    }
}
