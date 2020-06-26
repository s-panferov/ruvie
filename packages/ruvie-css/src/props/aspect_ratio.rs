pub enum AspectRatio {
    Auto,
}
impl std::fmt::Display for AspectRatio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AspectRatio::Auto => write!(f, "auto"),
        }
    }
}
impl crate::ValueFor<AspectRatio> for AspectRatio {}
impl crate::Attribute for AspectRatio {
    const NAME: &'static str = "aspect-ratio";
}
impl crate::StyleSheet {
    pub fn aspect_ratio<V: crate::ValueFor<AspectRatio>>(mut self, value: V) -> Self {
        self.rules.insert("aspect-ratio", value.value());
        self
    }
}
