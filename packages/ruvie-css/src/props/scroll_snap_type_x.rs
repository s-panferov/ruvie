pub enum ScrollSnapTypeX {
    Mandatory,
    None,
    Proximity,
}
impl std::fmt::Display for ScrollSnapTypeX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScrollSnapTypeX::Mandatory => write!(f, "mandatory"),
            ScrollSnapTypeX::None => write!(f, "none"),
            ScrollSnapTypeX::Proximity => write!(f, "proximity"),
        }
    }
}
impl crate::ValueFor<ScrollSnapTypeX> for ScrollSnapTypeX {}
impl crate::Attribute for ScrollSnapTypeX {
    const NAME: &'static str = "scroll-snap-type-x";
}
impl crate::StyleSheet {
    pub fn scroll_snap_type_x<V: crate::ValueFor<ScrollSnapTypeX>>(mut self, value: V) -> Self {
        self.rules.insert("scroll-snap-type-x", value.value());
        self
    }
}
