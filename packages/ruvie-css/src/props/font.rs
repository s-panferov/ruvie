pub enum Font {}
impl crate::Attribute for Font {
    const NAME: &'static str = "font";
}
impl crate::StyleSheet {
    pub fn font<V: crate::ValueFor<Font>>(mut self, value: V) -> Self {
        self.rules.insert("font", value.value());
        self
    }
}
