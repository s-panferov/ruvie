pub enum GridAutoRows {}
impl crate::Attribute for GridAutoRows {
    const NAME: &'static str = "grid-auto-rows";
}
impl crate::StyleSheet {
    pub fn grid_auto_rows<V: crate::ValueFor<GridAutoRows>>(mut self, value: V) -> Self {
        self.rules.insert("grid-auto-rows", value.value());
        self
    }
}
