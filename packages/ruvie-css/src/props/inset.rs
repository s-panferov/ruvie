pub enum Inset {}
impl crate::Attribute for Inset {
    const NAME: &'static str = "inset";
}
impl crate::StyleSheet {
    pub fn inset<V: crate::ValueFor<Inset>>(mut self, value: V) -> Self {
        self.rules.insert("inset", value.value());
        self
    }
}
