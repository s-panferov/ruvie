pub enum CounterSet {}
impl crate::Attribute for CounterSet {
    const NAME: &'static str = "counter-set";
}
impl crate::StyleSheet {
    pub fn counter_set<V: crate::ValueFor<CounterSet>>(mut self, value: V) -> Self {
        self.rules.insert("counter-set", value.value());
        self
    }
}
