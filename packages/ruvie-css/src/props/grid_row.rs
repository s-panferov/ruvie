pub enum GridRow {}
impl crate::Attribute for GridRow {
    const NAME: &'static str = "grid-row";
}
impl crate::StyleSheet {
    pub fn grid_row<V: crate::ValueFor<GridRow>>(mut self, value: V) -> Self {
        self.rules.insert("grid-row", value.value());
        self
    }
}
