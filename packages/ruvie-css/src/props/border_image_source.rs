pub enum BorderImageSource {
    None,
}
impl std::fmt::Display for BorderImageSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BorderImageSource::None => write!(f, "none"),
        }
    }
}
impl crate::ValueFor<BorderImageSource> for BorderImageSource {}
impl crate::Attribute for BorderImageSource {
    const NAME: &'static str = "border-image-source";
}
impl crate::StyleSheet {
    pub fn border_image_source<V: crate::ValueFor<BorderImageSource>>(mut self, value: V) -> Self {
        self.rules.insert("border-image-source", value.value());
        self
    }
}
