pub enum BackgroundSize {}
impl crate::Attribute for BackgroundSize {
    const NAME: &'static str = "background-size";
}
impl crate::StyleSheet {
    pub fn background_size<V: crate::ValueFor<BackgroundSize>>(mut self, value: V) -> Self {
        self.rules.insert("background-size", value.value());
        self
    }
}
