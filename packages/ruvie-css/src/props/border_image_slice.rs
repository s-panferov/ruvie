pub enum BorderImageSlice {}
impl crate::Attribute for BorderImageSlice {
    const NAME: &'static str = "border-image-slice";
}
impl crate::StyleSheet {
    pub fn border_image_slice<V: crate::ValueFor<BorderImageSlice>>(mut self, value: V) -> Self {
        self.rules.insert("border-image-slice", value.value());
        self
    }
}
