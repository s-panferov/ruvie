pub enum MaskBorderSource {
    None,
}
impl std::fmt::Display for MaskBorderSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MaskBorderSource::None => write!(f, "none"),
        }
    }
}
impl crate::ValueFor<MaskBorderSource> for MaskBorderSource {}
impl crate::Attribute for MaskBorderSource {
    const NAME: &'static str = "mask-border-source";
}
impl crate::StyleSheet {
    pub fn mask_border_source<V: crate::ValueFor<MaskBorderSource>>(mut self, value: V) -> Self {
        self.rules.insert("mask-border-source", value.value());
        self
    }
}
