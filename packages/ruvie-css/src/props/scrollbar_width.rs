pub enum ScrollbarWidth {
    Auto,
    None,
    Thin,
}
impl std::fmt::Display for ScrollbarWidth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScrollbarWidth::Auto => write!(f, "auto"),
            ScrollbarWidth::None => write!(f, "none"),
            ScrollbarWidth::Thin => write!(f, "thin"),
        }
    }
}
impl crate::ValueFor<ScrollbarWidth> for ScrollbarWidth {}
impl crate::Attribute for ScrollbarWidth {
    const NAME: &'static str = "scrollbar-width";
}
impl crate::StyleSheet {
    pub fn scrollbar_width<V: crate::ValueFor<ScrollbarWidth>>(mut self, value: V) -> Self {
        self.rules.insert("scrollbar-width", value.value());
        self
    }
}
