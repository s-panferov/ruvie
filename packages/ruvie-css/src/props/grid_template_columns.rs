pub enum GridTemplateColumns {}
impl crate::Attribute for GridTemplateColumns {
    const NAME: &'static str = "grid-template-columns";
}
impl crate::StyleSheet {
    pub fn grid_template_columns<V: crate::ValueFor<GridTemplateColumns>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("grid-template-columns", value.value());
        self
    }
}
