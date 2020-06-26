pub enum TransitionProperty {}
impl crate::Attribute for TransitionProperty {
    const NAME: &'static str = "transition-property";
}
impl crate::StyleSheet {
    pub fn transition_property<V: crate::ValueFor<TransitionProperty>>(mut self, value: V) -> Self {
        self.rules.insert("transition-property", value.value());
        self
    }
}
