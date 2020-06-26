pub enum Column {}
impl crate::Attribute for Column {
    const NAME: &'static str = "columns";
}
impl crate::StyleSheet {
    pub fn columns<V: crate::ValueFor<Column>>(mut self, value: V) -> Self {
        self.rules.insert("columns", value.value());
        self
    }
}
