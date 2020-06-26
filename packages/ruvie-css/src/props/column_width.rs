pub enum ColumnWidth {
    Auto,
}
impl std::fmt::Display for ColumnWidth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColumnWidth::Auto => write!(f, "auto"),
        }
    }
}
impl crate::ValueFor<ColumnWidth> for ColumnWidth {}
impl crate::Attribute for ColumnWidth {
    const NAME: &'static str = "column-width";
}
impl crate::StyleSheet {
    pub fn column_width<V: crate::ValueFor<ColumnWidth>>(mut self, value: V) -> Self {
        self.rules.insert("column-width", value.value());
        self
    }
}
impl crate::ValueFor<ColumnWidth> for crate::types::length::Length {}
