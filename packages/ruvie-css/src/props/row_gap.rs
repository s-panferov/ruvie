pub enum RowGap {
    Normal,
}
impl std::fmt::Display for RowGap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RowGap::Normal => write!(f, "normal"),
        }
    }
}
impl crate::ValueFor<RowGap> for RowGap {}
impl crate::Attribute for RowGap {
    const NAME: &'static str = "row-gap";
}
impl crate::StyleSheet {
    pub fn row_gap<V: crate::ValueFor<RowGap>>(mut self, value: V) -> Self {
        self.rules.insert("row-gap", value.value());
        self
    }
}

impl crate::ValueFor<RowGap> for crate::types::length::Length {}
impl crate::ValueFor<RowGap> for crate::types::percentage::Percentage {}
