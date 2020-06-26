pub enum Rotate {}
impl crate::Attribute for Rotate {
    const NAME: &'static str = "rotate";
}
impl crate::StyleSheet {
    pub fn rotate<V: crate::ValueFor<Rotate>>(mut self, value: V) -> Self {
        self.rules.insert("rotate", value.value());
        self
    }
}
