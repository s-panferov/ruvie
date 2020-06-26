pub enum OverscrollBehavior {}
impl crate::Attribute for OverscrollBehavior {
    const NAME: &'static str = "overscroll-behavior";
}
impl crate::StyleSheet {
    pub fn overscroll_behavior<V: crate::ValueFor<OverscrollBehavior>>(mut self, value: V) -> Self {
        self.rules.insert("overscroll-behavior", value.value());
        self
    }
}
