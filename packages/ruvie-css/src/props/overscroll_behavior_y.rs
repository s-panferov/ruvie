pub enum OverscrollBehaviorY {
    Auto,
    Contain,
    None,
}
impl std::fmt::Display for OverscrollBehaviorY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OverscrollBehaviorY::Auto => write!(f, "auto"),
            OverscrollBehaviorY::Contain => write!(f, "contain"),
            OverscrollBehaviorY::None => write!(f, "none"),
        }
    }
}
impl crate::ValueFor<OverscrollBehaviorY> for OverscrollBehaviorY {}
impl crate::Attribute for OverscrollBehaviorY {
    const NAME: &'static str = "overscroll-behavior-y";
}
impl crate::StyleSheet {
    pub fn overscroll_behavior_y<V: crate::ValueFor<OverscrollBehaviorY>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("overscroll-behavior-y", value.value());
        self
    }
}
