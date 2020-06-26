pub enum MaskBorderMode {
    Alpha,
    Luminance,
}
impl std::fmt::Display for MaskBorderMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MaskBorderMode::Alpha => write!(f, "alpha"),
            MaskBorderMode::Luminance => write!(f, "luminance"),
        }
    }
}
impl crate::ValueFor<MaskBorderMode> for MaskBorderMode {}
impl crate::Attribute for MaskBorderMode {
    const NAME: &'static str = "mask-border-mode";
}
impl crate::StyleSheet {
    pub fn mask_border_mode<V: crate::ValueFor<MaskBorderMode>>(mut self, value: V) -> Self {
        self.rules.insert("mask-border-mode", value.value());
        self
    }
}
