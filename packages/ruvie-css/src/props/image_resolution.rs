pub enum ImageResolution {}
impl crate::Attribute for ImageResolution {
    const NAME: &'static str = "image-resolution";
}
impl crate::StyleSheet {
    pub fn image_resolution<V: crate::ValueFor<ImageResolution>>(mut self, value: V) -> Self {
        self.rules.insert("image-resolution", value.value());
        self
    }
}
