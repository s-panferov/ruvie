pub enum BackgroundClip {}
impl crate::Attribute for BackgroundClip {
    const NAME: &'static str = "background-clip";
}
impl crate::StyleSheet {
    pub fn background_clip<V: crate::ValueFor<BackgroundClip>>(mut self, value: V) -> Self {
        self.rules.insert("background-clip", value.value());
        self
    }
}
