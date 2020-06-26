pub enum WritingMode {
    HorizontalTb,
    SidewaysLr,
    SidewaysRl,
    VerticalLr,
    VerticalRl,
}
impl std::fmt::Display for WritingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WritingMode::HorizontalTb => write!(f, "horizontal-tb"),
            WritingMode::SidewaysLr => write!(f, "sideways-lr"),
            WritingMode::SidewaysRl => write!(f, "sideways-rl"),
            WritingMode::VerticalLr => write!(f, "vertical-lr"),
            WritingMode::VerticalRl => write!(f, "vertical-rl"),
        }
    }
}
impl crate::ValueFor<WritingMode> for WritingMode {}
impl crate::Attribute for WritingMode {
    const NAME: &'static str = "writing-mode";
}
impl crate::StyleSheet {
    pub fn writing_mode<V: crate::ValueFor<WritingMode>>(mut self, value: V) -> Self {
        self.rules.insert("writing-mode", value.value());
        self
    }
}
