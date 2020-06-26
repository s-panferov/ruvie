pub enum GridRowGap {}
impl crate::Attribute for GridRowGap {
    const NAME: &'static str = "grid-row-gap";
}
impl crate::StyleSheet {
    pub fn grid_row_gap<V: crate::ValueFor<GridRowGap>>(mut self, value: V) -> Self {
        self.rules.insert("grid-row-gap", value.value());
        self
    }
}

impl crate::ValueFor<GridRowGap> for crate::types::length::Length {}
impl crate::ValueFor<GridRowGap> for crate::types::percentage::Percentage {}
