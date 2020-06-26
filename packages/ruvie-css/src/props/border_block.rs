pub enum BorderBlock {}
impl crate::Attribute for BorderBlock {
    const NAME: &'static str = "border-block";
}
impl crate::StyleSheet {
    pub fn border_block<V: crate::ValueFor<BorderBlock>>(mut self, value: V) -> Self {
        self.rules.insert("border-block", value.value());
        self
    }
}
