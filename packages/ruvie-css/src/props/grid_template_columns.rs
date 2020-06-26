pub enum GridTemplateColumn {}
impl crate::Attribute for GridTemplateColumn {
    const NAME: &'static str = "grid-template-columns";
}
impl crate::StyleSheet {
    pub fn grid_template_columns<V: crate::ValueFor<GridTemplateColumn>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("grid-template-columns", value.value());
        self
    }
}
