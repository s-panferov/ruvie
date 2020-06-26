pub enum ScrollSnapPointsX {
    None,
}
impl std::fmt::Display for ScrollSnapPointsX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScrollSnapPointsX::None => write!(f, "none"),
        }
    }
}
impl crate::ValueFor<ScrollSnapPointsX> for ScrollSnapPointsX {}
impl crate::Attribute for ScrollSnapPointsX {
    const NAME: &'static str = "scroll-snap-points-x";
}
impl crate::StyleSheet {
    pub fn scroll_snap_points_x<V: crate::ValueFor<ScrollSnapPointsX>>(mut self, value: V) -> Self {
        self.rules.insert("scroll-snap-points-x", value.value());
        self
    }
}
