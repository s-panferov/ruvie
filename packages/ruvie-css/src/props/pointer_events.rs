pub enum PointerEvent {}
impl crate::Attribute for PointerEvent {
    const NAME: &'static str = "pointer-events";
}
impl crate::StyleSheet {
    pub fn pointer_events<V: crate::ValueFor<PointerEvent>>(mut self, value: V) -> Self {
        self.rules.insert("pointer-events", value.value());
        self
    }
}
