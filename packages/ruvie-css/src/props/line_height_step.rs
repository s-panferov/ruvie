pub enum LineHeightStep {}
impl crate::Attribute for LineHeightStep {
    const NAME: &'static str = "line-height-step";
}
impl crate::StyleSheet {
    pub fn line_height_step<V: crate::ValueFor<LineHeightStep>>(mut self, value: V) -> Self {
        self.rules.insert("line-height-step", value.value());
        self
    }
}
impl crate::ValueFor<LineHeightStep> for crate::types::length::Length {}
