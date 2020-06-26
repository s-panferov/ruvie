pub enum GridAutoColumns {}
impl crate::Attribute for GridAutoColumns {
    const NAME: &'static str = "grid-auto-columns";
}
impl crate::StyleSheet {
    pub fn grid_auto_columns<V: crate::ValueFor<GridAutoColumns>>(mut self, value: V) -> Self {
        self.rules.insert("grid-auto-columns", value.value());
        self
    }
}
