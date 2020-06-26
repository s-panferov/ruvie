pub enum TextOrientation {
    Mixed,
    Sideway,
    Upright,
}
impl std::fmt::Display for TextOrientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextOrientation::Mixed => write!(f, "mixed"),
            TextOrientation::Sideway => write!(f, "sideways"),
            TextOrientation::Upright => write!(f, "upright"),
        }
    }
}
impl crate::ValueFor<TextOrientation> for TextOrientation {}
impl crate::Attribute for TextOrientation {
    const NAME: &'static str = "text-orientation";
}
impl crate::StyleSheet {
    pub fn text_orientation<V: crate::ValueFor<TextOrientation>>(mut self, value: V) -> Self {
        self.rules.insert("text-orientation", value.value());
        self
    }
}
