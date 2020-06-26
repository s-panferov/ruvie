pub enum BackgroundPositionX {}
impl crate::Attribute for BackgroundPositionX {
    const NAME: &'static str = "background-position-x";
}
impl crate::StyleSheet {
    pub fn background_position_x<V: crate::ValueFor<BackgroundPositionX>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("background-position-x", value.value());
        self
    }
}
