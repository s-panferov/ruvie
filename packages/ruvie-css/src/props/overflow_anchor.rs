pub enum OverflowAnchor {
    Auto,
    None,
}
impl std::fmt::Display for OverflowAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OverflowAnchor::Auto => write!(f, "auto"),
            OverflowAnchor::None => write!(f, "none"),
        }
    }
}
impl crate::ValueFor<OverflowAnchor> for OverflowAnchor {}
impl crate::Attribute for OverflowAnchor {
    const NAME: &'static str = "overflow-anchor";
}
impl crate::StyleSheet {
    pub fn overflow_anchor<V: crate::ValueFor<OverflowAnchor>>(mut self, value: V) -> Self {
        self.rules.insert("overflow-anchor", value.value());
        self
    }
}
