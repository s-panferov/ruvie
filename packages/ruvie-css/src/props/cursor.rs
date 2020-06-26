pub enum Cursor {}
impl crate::Attribute for Cursor {
    const NAME: &'static str = "cursor";
}
impl crate::StyleSheet {
    pub fn cursor<V: crate::ValueFor<Cursor>>(mut self, value: V) -> Self {
        self.rules.insert("cursor", value.value());
        self
    }
}
