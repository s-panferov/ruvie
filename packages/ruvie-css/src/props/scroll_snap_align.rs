pub enum ScrollSnapAlign {}
impl crate::Attribute for ScrollSnapAlign {
    const NAME: &'static str = "scroll-snap-align";
}
impl crate::StyleSheet {
    pub fn scroll_snap_align<V: crate::ValueFor<ScrollSnapAlign>>(mut self, value: V) -> Self {
        self.rules.insert("scroll-snap-align", value.value());
        self
    }
}
