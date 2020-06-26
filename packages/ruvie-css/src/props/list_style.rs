pub enum ListStyle {}
impl crate::Attribute for ListStyle {
    const NAME: &'static str = "list-style";
}
impl crate::StyleSheet {
    pub fn list_style<V: crate::ValueFor<ListStyle>>(mut self, value: V) -> Self {
        self.rules.insert("list-style", value.value());
        self
    }
}
