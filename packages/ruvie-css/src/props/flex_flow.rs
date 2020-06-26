pub enum FlexFlow {}
impl crate::Attribute for FlexFlow {
    const NAME: &'static str = "flex-flow";
}
impl crate::StyleSheet {
    pub fn flex_flow<V: crate::ValueFor<FlexFlow>>(mut self, value: V) -> Self {
        self.rules.insert("flex-flow", value.value());
        self
    }
}
