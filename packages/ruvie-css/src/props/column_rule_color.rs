pub enum ColumnRuleColor {}
impl crate::Attribute for ColumnRuleColor {
    const NAME: &'static str = "column-rule-color";
}
impl crate::StyleSheet {
    pub fn column_rule_color<V: crate::ValueFor<ColumnRuleColor>>(mut self, value: V) -> Self {
        self.rules.insert("column-rule-color", value.value());
        self
    }
}

impl crate::ValueFor<ColumnRuleColor> for crate::types::color::Color {}
