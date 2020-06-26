pub enum GridTemplateAreas {}
impl crate::Attribute for GridTemplateAreas {
    const NAME: &'static str = "grid-template-areas";
}
impl crate::StyleSheet {
    pub fn grid_template_areas<V: crate::ValueFor<GridTemplateAreas>>(mut self, value: V) -> Self {
        self.rules.insert("grid-template-areas", value.value());
        self
    }
}
