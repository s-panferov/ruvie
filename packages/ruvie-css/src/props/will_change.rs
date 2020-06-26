pub enum WillChange {}
impl crate::Attribute for WillChange {
    const NAME: &'static str = "will-change";
}
impl crate::StyleSheet {
    pub fn will_change<V: crate::ValueFor<WillChange>>(mut self, value: V) -> Self {
        self.rules.insert("will-change", value.value());
        self
    }
}
