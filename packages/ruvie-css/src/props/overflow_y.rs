pub enum OverflowY {
    Auto,
    Clip,
    Hidden,
    Scroll,
    Visible,
}
impl std::fmt::Display for OverflowY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OverflowY::Auto => write!(f, "auto"),
            OverflowY::Clip => write!(f, "clip"),
            OverflowY::Hidden => write!(f, "hidden"),
            OverflowY::Scroll => write!(f, "scroll"),
            OverflowY::Visible => write!(f, "visible"),
        }
    }
}
impl crate::ValueFor<OverflowY> for OverflowY {}
impl crate::Attribute for OverflowY {
    const NAME: &'static str = "overflow-y";
}
impl crate::StyleSheet {
    pub fn overflow_y<V: crate::ValueFor<OverflowY>>(mut self, value: V) -> Self {
        self.rules.insert("overflow-y", value.value());
        self
    }
}
