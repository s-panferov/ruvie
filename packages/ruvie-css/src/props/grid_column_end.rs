pub enum GridColumnEnd {}
impl crate::Attribute for GridColumnEnd {
    const NAME: &'static str = "grid-column-end";
}
impl crate::StyleSheet {
    pub fn grid_column_end<V: crate::ValueFor<GridColumnEnd>>(mut self, value: V) -> Self {
        self.rules.insert("grid-column-end", value.value());
        self
    }
}
