pub enum JustifyContent {}
impl crate::Attribute for JustifyContent {
    const NAME: &'static str = "justify-content";
}
impl crate::StyleSheet {
    pub fn justify_content<V: crate::ValueFor<JustifyContent>>(mut self, value: V) -> Self {
        self.rules.insert("justify-content", value.value());
        self
    }
}
