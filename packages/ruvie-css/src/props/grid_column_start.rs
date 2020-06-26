pub enum GridColumnStart {}
impl crate::Attribute for GridColumnStart {
    const NAME: &'static str = "grid-column-start";
}
impl crate::StyleSheet {
    pub fn grid_column_start<V: crate::ValueFor<GridColumnStart>>(mut self, value: V) -> Self {
        self.rules.insert("grid-column-start", value.value());
        self
    }
}
