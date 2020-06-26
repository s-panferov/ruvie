pub enum ColumnGap {
    Normal,
}
impl std::fmt::Display for ColumnGap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColumnGap::Normal => write!(f, "normal"),
        }
    }
}
impl crate::ValueFor<ColumnGap> for ColumnGap {}
impl crate::Attribute for ColumnGap {
    const NAME: &'static str = "column-gap";
}
impl crate::StyleSheet {
    pub fn column_gap<V: crate::ValueFor<ColumnGap>>(mut self, value: V) -> Self {
        self.rules.insert("column-gap", value.value());
        self
    }
}

impl crate::ValueFor<ColumnGap> for crate::types::length::Length {}
impl crate::ValueFor<ColumnGap> for crate::types::percentage::Percentage {}
