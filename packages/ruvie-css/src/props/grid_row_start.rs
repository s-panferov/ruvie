pub enum GridRowStart {}
impl crate::Attribute for GridRowStart {
    const NAME: &'static str = "grid-row-start";
}
impl crate::StyleSheet {
    pub fn grid_row_start<V: crate::ValueFor<GridRowStart>>(mut self, value: V) -> Self {
        self.rules.insert("grid-row-start", value.value());
        self
    }
}
