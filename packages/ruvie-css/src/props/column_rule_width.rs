pub enum ColumnRuleWidth {}
impl crate::Attribute for ColumnRuleWidth {
    const NAME: &'static str = "column-rule-width";
}
impl crate::StyleSheet {
    pub fn column_rule_width<V: crate::ValueFor<ColumnRuleWidth>>(mut self, value: V) -> Self {
        self.rules.insert("column-rule-width", value.value());
        self
    }
}
