pub enum GridColumnGap {}
impl crate::Attribute for GridColumnGap {
    const NAME: &'static str = "grid-column-gap";
}
impl crate::StyleSheet {
    pub fn grid_column_gap<V: crate::ValueFor<GridColumnGap>>(mut self, value: V) -> Self {
        self.rules.insert("grid-column-gap", value.value());
        self
    }
}

impl crate::ValueFor<GridColumnGap> for crate::types::length::Length {}
impl crate::ValueFor<GridColumnGap> for crate::types::percentage::Percentage {}
