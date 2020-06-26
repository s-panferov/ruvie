pub enum BorderImagelouse {}
impl crate::Attribute for BorderImagelouse {
    const NAME: &'static str = "border-image-slice";
}
impl crate::StyleSheet {
    pub fn border_image_slice<V: crate::ValueFor<BorderImagelouse>>(mut self, value: V) -> Self {
        self.rules.insert("border-image-slice", value.value());
        self
    }
}
