pub enum TransitionDuration {}
impl crate::Attribute for TransitionDuration {
    const NAME: &'static str = "transition-duration";
}
impl crate::StyleSheet {
    pub fn transition_duration<V: crate::ValueFor<TransitionDuration>>(mut self, value: V) -> Self {
        self.rules.insert("transition-duration", value.value());
        self
    }
}
