pub enum OverscrollBehaviorX {
    Auto,
    Contain,
    None,
}
impl std::fmt::Display for OverscrollBehaviorX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OverscrollBehaviorX::Auto => write!(f, "auto"),
            OverscrollBehaviorX::Contain => write!(f, "contain"),
            OverscrollBehaviorX::None => write!(f, "none"),
        }
    }
}
impl crate::ValueFor<OverscrollBehaviorX> for OverscrollBehaviorX {}
impl crate::Attribute for OverscrollBehaviorX {
    const NAME: &'static str = "overscroll-behavior-x";
}
impl crate::StyleSheet {
    pub fn overscroll_behavior_x<V: crate::ValueFor<OverscrollBehaviorX>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("overscroll-behavior-x", value.value());
        self
    }
}
