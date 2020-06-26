pub enum GridAutoFlow {}
impl crate::Attribute for GridAutoFlow {
    const NAME: &'static str = "grid-auto-flow";
}
impl crate::StyleSheet {
    pub fn grid_auto_flow<V: crate::ValueFor<GridAutoFlow>>(mut self, value: V) -> Self {
        self.rules.insert("grid-auto-flow", value.value());
        self
    }
}
