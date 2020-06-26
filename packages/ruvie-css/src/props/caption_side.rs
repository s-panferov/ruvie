pub enum CaptionSide {
    BlockEnd,
    BlockStart,
    Bottom,
    InlineEnd,
    InlineStart,
    Top,
}
impl std::fmt::Display for CaptionSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CaptionSide::BlockEnd => write!(f, "block-end"),
            CaptionSide::BlockStart => write!(f, "block-start"),
            CaptionSide::Bottom => write!(f, "bottom"),
            CaptionSide::InlineEnd => write!(f, "inline-end"),
            CaptionSide::InlineStart => write!(f, "inline-start"),
            CaptionSide::Top => write!(f, "top"),
        }
    }
}
impl crate::ValueFor<CaptionSide> for CaptionSide {}
impl crate::Attribute for CaptionSide {
    const NAME: &'static str = "caption-side";
}
impl crate::StyleSheet {
    pub fn caption_side<V: crate::ValueFor<CaptionSide>>(mut self, value: V) -> Self {
        self.rules.insert("caption-side", value.value());
        self
    }
}
