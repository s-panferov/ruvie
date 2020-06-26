pub enum ColumnRuleStyle {}
impl crate::Attribute for ColumnRuleStyle {
    const NAME: &'static str = "column-rule-style";
}
impl crate::StyleSheet {
    pub fn column_rule_style<V: crate::ValueFor<ColumnRuleStyle>>(mut self, value: V) -> Self {
        self.rules.insert("column-rule-style", value.value());
        self
    }
}
