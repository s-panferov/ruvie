pub enum ScrollSnapDestination {}
impl crate::Attribute for ScrollSnapDestination {
    const NAME: &'static str = "scroll-snap-destination";
}
impl crate::StyleSheet {
    pub fn scroll_snap_destination<V: crate::ValueFor<ScrollSnapDestination>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("scroll-snap-destination", value.value());
        self
    }
}
