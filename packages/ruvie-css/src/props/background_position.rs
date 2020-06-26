pub enum BackgroundPosition {}
impl crate::Attribute for BackgroundPosition {
    const NAME: &'static str = "background-position";
}
impl crate::StyleSheet {
    pub fn background_position<V: crate::ValueFor<BackgroundPosition>>(mut self, value: V) -> Self {
        self.rules.insert("background-position", value.value());
        self
    }
}
