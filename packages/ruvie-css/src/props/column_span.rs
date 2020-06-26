pub enum ColumnSpan {
    All,
    None,
}
impl std::fmt::Display for ColumnSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColumnSpan::All => write!(f, "all"),
            ColumnSpan::None => write!(f, "none"),
        }
    }
}
impl crate::ValueFor<ColumnSpan> for ColumnSpan {}
impl crate::Attribute for ColumnSpan {
    const NAME: &'static str = "column-span";
}
impl crate::StyleSheet {
    pub fn column_span<V: crate::ValueFor<ColumnSpan>>(mut self, value: V) -> Self {
        self.rules.insert("column-span", value.value());
        self
    }
}
