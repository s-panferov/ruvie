pub enum ColumnFill {
    Auto,
    Balance,
    BalanceAll,
}
impl std::fmt::Display for ColumnFill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColumnFill::Auto => write!(f, "auto"),
            ColumnFill::Balance => write!(f, "balance"),
            ColumnFill::BalanceAll => write!(f, "balance-all"),
        }
    }
}
impl crate::ValueFor<ColumnFill> for ColumnFill {}
impl crate::Attribute for ColumnFill {
    const NAME: &'static str = "column-fill";
}
impl crate::StyleSheet {
    pub fn column_fill<V: crate::ValueFor<ColumnFill>>(mut self, value: V) -> Self {
        self.rules.insert("column-fill", value.value());
        self
    }
}
