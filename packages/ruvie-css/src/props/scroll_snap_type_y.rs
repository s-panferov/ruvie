pub enum ScrollSnapTypeY {
    Mandatory,
    None,
    Proximity,
}
impl std::fmt::Display for ScrollSnapTypeY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScrollSnapTypeY::Mandatory => write!(f, "mandatory"),
            ScrollSnapTypeY::None => write!(f, "none"),
            ScrollSnapTypeY::Proximity => write!(f, "proximity"),
        }
    }
}
impl crate::ValueFor<ScrollSnapTypeY> for ScrollSnapTypeY {}
impl crate::Attribute for ScrollSnapTypeY {
    const NAME: &'static str = "scroll-snap-type-y";
}
impl crate::StyleSheet {
    pub fn scroll_snap_type_y<V: crate::ValueFor<ScrollSnapTypeY>>(mut self, value: V) -> Self {
        self.rules.insert("scroll-snap-type-y", value.value());
        self
    }
}
