pub enum GridArea {}
impl crate::Attribute for GridArea {
    const NAME: &'static str = "grid-area";
}
impl crate::StyleSheet {
    pub fn grid_area<V: crate::ValueFor<GridArea>>(mut self, value: V) -> Self {
        self.rules.insert("grid-area", value.value());
        self
    }
}
