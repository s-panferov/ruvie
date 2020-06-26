pub enum Grid {}
impl crate::Attribute for Grid {
    const NAME: &'static str = "grid";
}
impl crate::StyleSheet {
    pub fn grid<V: crate::ValueFor<Grid>>(mut self, value: V) -> Self {
        self.rules.insert("grid", value.value());
        self
    }
}
