pub enum BorderImageWidth {}
impl crate::Attribute for BorderImageWidth {
    const NAME: &'static str = "border-image-width";
}
impl crate::StyleSheet {
    pub fn border_image_width<V: crate::ValueFor<BorderImageWidth>>(mut self, value: V) -> Self {
        self.rules.insert("border-image-width", value.value());
        self
    }
}
