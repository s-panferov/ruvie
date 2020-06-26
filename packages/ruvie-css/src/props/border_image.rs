pub enum BorderImage {}
impl crate::Attribute for BorderImage {
    const NAME: &'static str = "border-image";
}
impl crate::StyleSheet {
    pub fn border_image<V: crate::ValueFor<BorderImage>>(mut self, value: V) -> Self {
        self.rules.insert("border-image", value.value());
        self
    }
}
