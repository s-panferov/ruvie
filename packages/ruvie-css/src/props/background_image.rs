pub enum BackgroundImage {}
impl crate::Attribute for BackgroundImage {
    const NAME: &'static str = "background-image";
}
impl crate::StyleSheet {
    pub fn background_image<V: crate::ValueFor<BackgroundImage>>(mut self, value: V) -> Self {
        self.rules.insert("background-image", value.value());
        self
    }
}
