pub enum Outline {}
impl crate::Attribute for Outline {
    const NAME: &'static str = "outline";
}
impl crate::StyleSheet {
    pub fn outline<V: crate::ValueFor<Outline>>(mut self, value: V) -> Self {
        self.rules.insert("outline", value.value());
        self
    }
}
