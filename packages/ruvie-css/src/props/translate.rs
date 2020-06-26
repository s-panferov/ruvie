pub enum Translate {}
impl crate::Attribute for Translate {
    const NAME: &'static str = "translate";
}
impl crate::StyleSheet {
    pub fn translate<V: crate::ValueFor<Translate>>(mut self, value: V) -> Self {
        self.rules.insert("translate", value.value());
        self
    }
}
