pub enum Background {}
impl crate::Attribute for Background {
    const NAME: &'static str = "background";
}
impl crate::StyleSheet {
    pub fn background<V: crate::ValueFor<Background>>(mut self, value: V) -> Self {
        self.rules.insert("background", value.value());
        self
    }
}
