pub enum BackgroundRepeat {}
impl crate::Attribute for BackgroundRepeat {
    const NAME: &'static str = "background-repeat";
}
impl crate::StyleSheet {
    pub fn background_repeat<V: crate::ValueFor<BackgroundRepeat>>(mut self, value: V) -> Self {
        self.rules.insert("background-repeat", value.value());
        self
    }
}
