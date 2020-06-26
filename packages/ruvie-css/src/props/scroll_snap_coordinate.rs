pub enum ScrollSnapCoordinate {}
impl crate::Attribute for ScrollSnapCoordinate {
    const NAME: &'static str = "scroll-snap-coordinate";
}
impl crate::StyleSheet {
    pub fn scroll_snap_coordinate<V: crate::ValueFor<ScrollSnapCoordinate>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("scroll-snap-coordinate", value.value());
        self
    }
}
