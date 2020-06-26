pub enum ListStylePosition {
    Inside,
    Outside,
}
impl std::fmt::Display for ListStylePosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListStylePosition::Inside => write!(f, "inside"),
            ListStylePosition::Outside => write!(f, "outside"),
        }
    }
}
impl crate::ValueFor<ListStylePosition> for ListStylePosition {}
impl crate::Attribute for ListStylePosition {
    const NAME: &'static str = "list-style-position";
}
impl crate::StyleSheet {
    pub fn list_style_position<V: crate::ValueFor<ListStylePosition>>(mut self, value: V) -> Self {
        self.rules.insert("list-style-position", value.value());
        self
    }
}
