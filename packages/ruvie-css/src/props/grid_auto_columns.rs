pub enum GridAutoColumn {}
impl crate::Attribute for GridAutoColumn {
    const NAME: &'static str = "grid-auto-columns";
}
impl crate::StyleSheet {
    pub fn grid_auto_columns<V: crate::ValueFor<GridAutoColumn>>(mut self, value: V) -> Self {
        self.rules.insert("grid-auto-columns", value.value());
        self
    }
}
