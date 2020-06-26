pub enum GridRowEnd {}
impl crate::Attribute for GridRowEnd {
    const NAME: &'static str = "grid-row-end";
}
impl crate::StyleSheet {
    pub fn grid_row_end<V: crate::ValueFor<GridRowEnd>>(mut self, value: V) -> Self {
        self.rules.insert("grid-row-end", value.value());
        self
    }
}
