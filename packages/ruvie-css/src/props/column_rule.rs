pub enum ColumnRule {}
impl crate::Attribute for ColumnRule {
    const NAME: &'static str = "column-rule";
}
impl crate::StyleSheet {
    pub fn column_rule<V: crate::ValueFor<ColumnRule>>(mut self, value: V) -> Self {
        self.rules.insert("column-rule", value.value());
        self
    }
}
