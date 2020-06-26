pub enum Gap {}
impl crate::Attribute for Gap {
    const NAME: &'static str = "gap";
}
impl crate::StyleSheet {
    pub fn gap<V: crate::ValueFor<Gap>>(mut self, value: V) -> Self {
        self.rules.insert("gap", value.value());
        self
    }
}
