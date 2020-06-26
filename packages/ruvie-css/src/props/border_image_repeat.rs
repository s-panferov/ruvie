pub enum BorderImageRepeat {}
impl crate::Attribute for BorderImageRepeat {
    const NAME: &'static str = "border-image-repeat";
}
impl crate::StyleSheet {
    pub fn border_image_repeat<V: crate::ValueFor<BorderImageRepeat>>(mut self, value: V) -> Self {
        self.rules.insert("border-image-repeat", value.value());
        self
    }
}
