pub enum ColumnCount {
    Auto,
}
impl std::fmt::Display for ColumnCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColumnCount::Auto => write!(f, "auto"),
        }
    }
}
impl crate::ValueFor<ColumnCount> for ColumnCount {}
impl crate::Attribute for ColumnCount {
    const NAME: &'static str = "column-count";
}
impl crate::StyleSheet {
    pub fn column_count<V: crate::ValueFor<ColumnCount>>(mut self, value: V) -> Self {
        self.rules.insert("column-count", value.value());
        self
    }
}

impl crate::ValueFor<ColumnCount> for usize {}
impl crate::ValueFor<ColumnCount> for isize {}
