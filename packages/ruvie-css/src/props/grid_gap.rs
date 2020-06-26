pub enum GridGap {}
impl crate::Attribute for GridGap {
    const NAME: &'static str = "grid-gap";
}
impl crate::StyleSheet {
    pub fn grid_gap<V: crate::ValueFor<GridGap>>(mut self, value: V) -> Self {
        self.rules.insert("grid-gap", value.value());
        self
    }
}
