pub enum BackgroundPositionY {}
impl crate::Attribute for BackgroundPositionY {
    const NAME: &'static str = "background-position-y";
}
impl crate::StyleSheet {
    pub fn background_position_y<V: crate::ValueFor<BackgroundPositionY>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("background-position-y", value.value());
        self
    }
}
