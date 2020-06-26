pub enum OffsetAnchor {
    Auto,
}
impl std::fmt::Display for OffsetAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OffsetAnchor::Auto => write!(f, "auto"),
        }
    }
}
impl crate::ValueFor<OffsetAnchor> for OffsetAnchor {}
impl crate::Attribute for OffsetAnchor {
    const NAME: &'static str = "offset-anchor";
}
impl crate::StyleSheet {
    pub fn offset_anchor<V: crate::ValueFor<OffsetAnchor>>(mut self, value: V) -> Self {
        self.rules.insert("offset-anchor", value.value());
        self
    }
}
