pub enum JustifyItem {}
impl crate::Attribute for JustifyItem {
    const NAME: &'static str = "justify-items";
}
impl crate::StyleSheet {
    pub fn justify_items<V: crate::ValueFor<JustifyItem>>(mut self, value: V) -> Self {
        self.rules.insert("justify-items", value.value());
        self
    }
}
