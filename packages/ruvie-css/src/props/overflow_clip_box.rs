pub enum OverflowClipBox {
    ContentBox,
    PaddingBox,
}
impl std::fmt::Display for OverflowClipBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OverflowClipBox::ContentBox => write!(f, "content-box"),
            OverflowClipBox::PaddingBox => write!(f, "padding-box"),
        }
    }
}
impl crate::ValueFor<OverflowClipBox> for OverflowClipBox {}
impl crate::Attribute for OverflowClipBox {
    const NAME: &'static str = "overflow-clip-box";
}
impl crate::StyleSheet {
    pub fn overflow_clip_box<V: crate::ValueFor<OverflowClipBox>>(mut self, value: V) -> Self {
        self.rules.insert("overflow-clip-box", value.value());
        self
    }
}
