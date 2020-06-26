pub enum Columns {}
impl crate::Attribute for Columns {
    const NAME: &'static str = "columns";
}
impl crate::StyleSheet {
    pub fn columns<V: crate::ValueFor<Columns>>(mut self, value: V) -> Self {
        self.rules.insert("columns", value.value());
        self
    }
}
