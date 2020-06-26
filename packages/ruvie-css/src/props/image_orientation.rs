pub enum ImageOrientation {}
impl crate::Attribute for ImageOrientation {
    const NAME: &'static str = "image-orientation";
}
impl crate::StyleSheet {
    pub fn image_orientation<V: crate::ValueFor<ImageOrientation>>(mut self, value: V) -> Self {
        self.rules.insert("image-orientation", value.value());
        self
    }
}
