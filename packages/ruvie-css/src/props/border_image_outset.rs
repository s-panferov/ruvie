pub enum BorderImageOutset {}
impl crate::Attribute for BorderImageOutset {
    const NAME: &'static str = "border-image-outset";
}
impl crate::StyleSheet {
    pub fn border_image_outset<V: crate::ValueFor<BorderImageOutset>>(mut self, value: V) -> Self {
        self.rules.insert("border-image-outset", value.value());
        self
    }
}
