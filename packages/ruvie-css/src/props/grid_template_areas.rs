pub enum GridTemplateArea {}
impl crate::Attribute for GridTemplateArea {
    const NAME: &'static str = "grid-template-areas";
}
impl crate::StyleSheet {
    pub fn grid_template_areas<V: crate::ValueFor<GridTemplateArea>>(mut self, value: V) -> Self {
        self.rules.insert("grid-template-areas", value.value());
        self
    }
}
