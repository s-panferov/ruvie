pub enum GridTemplate {}
impl crate::Attribute for GridTemplate {
    const NAME: &'static str = "grid-template";
}
impl crate::StyleSheet {
    pub fn grid_template<V: crate::ValueFor<GridTemplate>>(mut self, value: V) -> Self {
        self.rules.insert("grid-template", value.value());
        self
    }
}
