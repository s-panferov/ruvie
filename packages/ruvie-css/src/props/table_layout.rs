pub enum TableLayout {
    Auto,
    Fixed,
}
impl std::fmt::Display for TableLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableLayout::Auto => write!(f, "auto"),
            TableLayout::Fixed => write!(f, "fixed"),
        }
    }
}
impl crate::ValueFor<TableLayout> for TableLayout {}
impl crate::Attribute for TableLayout {
    const NAME: &'static str = "table-layout";
}
impl crate::StyleSheet {
    pub fn table_layout<V: crate::ValueFor<TableLayout>>(mut self, value: V) -> Self {
        self.rules.insert("table-layout", value.value());
        self
    }
}
