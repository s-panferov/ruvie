pub enum OverscrollBehaviorInline {
    Auto,
    Contain,
    None,
}
impl std::fmt::Display for OverscrollBehaviorInline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OverscrollBehaviorInline::Auto => write!(f, "auto"),
            OverscrollBehaviorInline::Contain => write!(f, "contain"),
            OverscrollBehaviorInline::None => write!(f, "none"),
        }
    }
}
impl crate::ValueFor<OverscrollBehaviorInline> for OverscrollBehaviorInline {}
impl crate::Attribute for OverscrollBehaviorInline {
    const NAME: &'static str = "overscroll-behavior-inline";
}
impl crate::StyleSheet {
    pub fn overscroll_behavior_inline<V: crate::ValueFor<OverscrollBehaviorInline>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules
            .insert("overscroll-behavior-inline", value.value());
        self
    }
}
