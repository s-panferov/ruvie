pub enum TransitionTimingFunction {}
impl crate::Attribute for TransitionTimingFunction {
    const NAME: &'static str = "transition-timing-function";
}
impl crate::StyleSheet {
    pub fn transition_timing_function<V: crate::ValueFor<TransitionTimingFunction>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules
            .insert("transition-timing-function", value.value());
        self
    }
}
