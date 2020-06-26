pub enum GridTemplateRow {}
impl crate::Attribute for GridTemplateRow {
    const NAME: &'static str = "grid-template-rows";
}
impl crate::StyleSheet {
    pub fn grid_template_rows<V: crate::ValueFor<GridTemplateRow>>(mut self, value: V) -> Self {
        self.rules.insert("grid-template-rows", value.value());
        self
    }
}
