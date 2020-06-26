pub enum BackgroundOrigin {}
impl crate::Attribute for BackgroundOrigin {
    const NAME: &'static str = "background-origin";
}
impl crate::StyleSheet {
    pub fn background_origin<V: crate::ValueFor<BackgroundOrigin>>(mut self, value: V) -> Self {
        self.rules.insert("background-origin", value.value());
        self
    }
}
