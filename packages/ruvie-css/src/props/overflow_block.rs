pub enum OverflowBlock {
    Auto,
    Clip,
    Hidden,
    Scroll,
    Visible,
}
impl std::fmt::Display for OverflowBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OverflowBlock::Auto => write!(f, "auto"),
            OverflowBlock::Clip => write!(f, "clip"),
            OverflowBlock::Hidden => write!(f, "hidden"),
            OverflowBlock::Scroll => write!(f, "scroll"),
            OverflowBlock::Visible => write!(f, "visible"),
        }
    }
}
impl crate::ValueFor<OverflowBlock> for OverflowBlock {}
impl crate::Attribute for OverflowBlock {
    const NAME: &'static str = "overflow-block";
}
impl crate::StyleSheet {
    pub fn overflow_block<V: crate::ValueFor<OverflowBlock>>(mut self, value: V) -> Self {
        self.rules.insert("overflow-block", value.value());
        self
    }
}
