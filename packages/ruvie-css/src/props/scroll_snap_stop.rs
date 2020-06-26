pub enum ScrollSnapStop {
    Alway,
    Normal,
}
impl std::fmt::Display for ScrollSnapStop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScrollSnapStop::Alway => write!(f, "always"),
            ScrollSnapStop::Normal => write!(f, "normal"),
        }
    }
}
impl crate::ValueFor<ScrollSnapStop> for ScrollSnapStop {}
impl crate::Attribute for ScrollSnapStop {
    const NAME: &'static str = "scroll-snap-stop";
}
impl crate::StyleSheet {
    pub fn scroll_snap_stop<V: crate::ValueFor<ScrollSnapStop>>(mut self, value: V) -> Self {
        self.rules.insert("scroll-snap-stop", value.value());
        self
    }
}
