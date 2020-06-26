pub enum Transition {}
impl crate::Attribute for Transition {
    const NAME: &'static str = "transition";
}
impl crate::StyleSheet {
    pub fn transition<V: crate::ValueFor<Transition>>(mut self, value: V) -> Self {
        self.rules.insert("transition", value.value());
        self
    }
}
