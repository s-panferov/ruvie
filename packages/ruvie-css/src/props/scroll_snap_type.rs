pub enum ScrollSnapType {}
impl crate::Attribute for ScrollSnapType {
    const NAME: &'static str = "scroll-snap-type";
}
impl crate::StyleSheet {
    pub fn scroll_snap_type<V: crate::ValueFor<ScrollSnapType>>(mut self, value: V) -> Self {
        self.rules.insert("scroll-snap-type", value.value());
        self
    }
}
