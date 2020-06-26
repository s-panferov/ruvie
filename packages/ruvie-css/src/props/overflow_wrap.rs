pub enum OverflowWrap {
    Anywhere,
    BreakWord,
    Normal,
}
impl std::fmt::Display for OverflowWrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OverflowWrap::Anywhere => write!(f, "anywhere"),
            OverflowWrap::BreakWord => write!(f, "break-word"),
            OverflowWrap::Normal => write!(f, "normal"),
        }
    }
}
impl crate::ValueFor<OverflowWrap> for OverflowWrap {}
impl crate::Attribute for OverflowWrap {
    const NAME: &'static str = "overflow-wrap";
}
impl crate::StyleSheet {
    pub fn overflow_wrap<V: crate::ValueFor<OverflowWrap>>(mut self, value: V) -> Self {
        self.rules.insert("overflow-wrap", value.value());
        self
    }
}
