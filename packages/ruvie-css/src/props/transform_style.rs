pub enum TransformStyle {
    Flat,
    Preserve3D,
}
impl std::fmt::Display for TransformStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransformStyle::Flat => write!(f, "flat"),
            TransformStyle::Preserve3D => write!(f, "preserve-3d"),
        }
    }
}
impl crate::ValueFor<TransformStyle> for TransformStyle {}
impl crate::Attribute for TransformStyle {
    const NAME: &'static str = "transform-style";
}
impl crate::StyleSheet {
    pub fn transform_style<V: crate::ValueFor<TransformStyle>>(mut self, value: V) -> Self {
        self.rules.insert("transform-style", value.value());
        self
    }
}
