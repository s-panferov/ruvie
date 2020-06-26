pub enum OverscrollBehaviorBlock {
    Auto,
    Contain,
    None,
}
impl std::fmt::Display for OverscrollBehaviorBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OverscrollBehaviorBlock::Auto => write!(f, "auto"),
            OverscrollBehaviorBlock::Contain => write!(f, "contain"),
            OverscrollBehaviorBlock::None => write!(f, "none"),
        }
    }
}
impl crate::ValueFor<OverscrollBehaviorBlock> for OverscrollBehaviorBlock {}
impl crate::Attribute for OverscrollBehaviorBlock {
    const NAME: &'static str = "overscroll-behavior-block";
}
impl crate::StyleSheet {
    pub fn overscroll_behavior_block<V: crate::ValueFor<OverscrollBehaviorBlock>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules
            .insert("overscroll-behavior-block", value.value());
        self
    }
}
