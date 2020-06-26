pub enum JustifyItems {}
impl crate::Attribute for JustifyItems {
    const NAME: &'static str = "justify-items";
}
impl crate::StyleSheet {
    pub fn justify_items<V: crate::ValueFor<JustifyItems>>(mut self, value: V) -> Self {
        self.rules.insert("justify-items", value.value());
        self
    }
}
