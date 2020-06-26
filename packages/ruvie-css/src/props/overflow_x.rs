pub enum OverflowX {
    Auto,
    Clip,
    Hidden,
    Scroll,
    Visible,
}
impl std::fmt::Display for OverflowX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OverflowX::Auto => write!(f, "auto"),
            OverflowX::Clip => write!(f, "clip"),
            OverflowX::Hidden => write!(f, "hidden"),
            OverflowX::Scroll => write!(f, "scroll"),
            OverflowX::Visible => write!(f, "visible"),
        }
    }
}
impl crate::ValueFor<OverflowX> for OverflowX {}
impl crate::Attribute for OverflowX {
    const NAME: &'static str = "overflow-x";
}
impl crate::StyleSheet {
    pub fn overflow_x<V: crate::ValueFor<OverflowX>>(mut self, value: V) -> Self {
        self.rules.insert("overflow-x", value.value());
        self
    }
}
