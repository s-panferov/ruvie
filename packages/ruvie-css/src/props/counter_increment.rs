pub enum CounterIncrement {}
impl crate::Attribute for CounterIncrement {
    const NAME: &'static str = "counter-increment";
}
impl crate::StyleSheet {
    pub fn counter_increment<V: crate::ValueFor<CounterIncrement>>(mut self, value: V) -> Self {
        self.rules.insert("counter-increment", value.value());
        self
    }
}
