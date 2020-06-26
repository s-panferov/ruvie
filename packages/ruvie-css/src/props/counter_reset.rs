pub enum CounterReset {}
impl crate::Attribute for CounterReset {
    const NAME: &'static str = "counter-reset";
}
impl crate::StyleSheet {
    pub fn counter_reset<V: crate::ValueFor<CounterReset>>(mut self, value: V) -> Self {
        self.rules.insert("counter-reset", value.value());
        self
    }
}
