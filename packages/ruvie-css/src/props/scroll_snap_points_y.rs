pub enum ScrollSnapPointsY {
    None,
}
impl std::fmt::Display for ScrollSnapPointsY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScrollSnapPointsY::None => write!(f, "none"),
        }
    }
}
impl crate::ValueFor<ScrollSnapPointsY> for ScrollSnapPointsY {}
impl crate::Attribute for ScrollSnapPointsY {
    const NAME: &'static str = "scroll-snap-points-y";
}
impl crate::StyleSheet {
    pub fn scroll_snap_points_y<V: crate::ValueFor<ScrollSnapPointsY>>(mut self, value: V) -> Self {
        self.rules.insert("scroll-snap-points-y", value.value());
        self
    }
}
