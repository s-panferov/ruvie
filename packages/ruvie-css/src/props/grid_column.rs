pub enum GridColumn {}
impl crate::Attribute for GridColumn {
    const NAME: &'static str = "grid-column";
}
impl crate::StyleSheet {
    pub fn grid_column<V: crate::ValueFor<GridColumn>>(mut self, value: V) -> Self {
        self.rules.insert("grid-column", value.value());
        self
    }
}
