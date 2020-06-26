pub enum Content {}
impl crate::Attribute for Content {
    const NAME: &'static str = "content";
}
impl crate::StyleSheet {
    pub fn content<V: crate::ValueFor<Content>>(mut self, value: V) -> Self {
        self.rules.insert("content", value.value());
        self
    }
}
