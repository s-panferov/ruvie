pub enum GridTemplateRows {}
impl crate::Attribute for GridTemplateRows {
    const NAME: &'static str = "grid-template-rows";
}
impl crate::StyleSheet {
    pub fn grid_template_rows<V: crate::ValueFor<GridTemplateRows>>(mut self, value: V) -> Self {
        self.rules.insert("grid-template-rows", value.value());
        self
    }
}
