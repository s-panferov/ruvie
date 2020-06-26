pub enum GridAutoRow {}
impl crate::Attribute for GridAutoRow {
    const NAME: &'static str = "grid-auto-rows";
}
impl crate::StyleSheet {
    pub fn grid_auto_rows<V: crate::ValueFor<GridAutoRow>>(mut self, value: V) -> Self {
        self.rules.insert("grid-auto-rows", value.value());
        self
    }
}
