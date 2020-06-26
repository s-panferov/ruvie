pub enum OffsetPosition {
    Auto,
}
impl std::fmt::Display for OffsetPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OffsetPosition::Auto => write!(f, "auto"),
        }
    }
}
impl crate::ValueFor<OffsetPosition> for OffsetPosition {}
impl crate::Attribute for OffsetPosition {
    const NAME: &'static str = "offset-position";
}
impl crate::StyleSheet {
    pub fn offset_position<V: crate::ValueFor<OffsetPosition>>(mut self, value: V) -> Self {
        self.rules.insert("offset-position", value.value());
        self
    }
}
