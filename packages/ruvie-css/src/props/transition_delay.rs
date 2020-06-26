pub enum TransitionDelay {}
impl crate::Attribute for TransitionDelay {
    const NAME: &'static str = "transition-delay";
}
impl crate::StyleSheet {
    pub fn transition_delay<V: crate::ValueFor<TransitionDelay>>(mut self, value: V) -> Self {
        self.rules.insert("transition-delay", value.value());
        self
    }
}
