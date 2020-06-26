pub enum ScrollBehavior {
    Auto,
    Smooth,
}
impl std::fmt::Display for ScrollBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScrollBehavior::Auto => write!(f, "auto"),
            ScrollBehavior::Smooth => write!(f, "smooth"),
        }
    }
}
impl crate::ValueFor<ScrollBehavior> for ScrollBehavior {}
impl crate::Attribute for ScrollBehavior {
    const NAME: &'static str = "scroll-behavior";
}
impl crate::StyleSheet {
    pub fn scroll_behavior<V: crate::ValueFor<ScrollBehavior>>(mut self, value: V) -> Self {
        self.rules.insert("scroll-behavior", value.value());
        self
    }
}
