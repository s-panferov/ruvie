pub enum PointerEvents {}
impl crate::Attribute for PointerEvents {
    const NAME: &'static str = "pointer-events";
}
impl crate::StyleSheet {
    pub fn pointer_events<V: crate::ValueFor<PointerEvents>>(mut self, value: V) -> Self {
        self.rules.insert("pointer-events", value.value());
        self
    }
}
