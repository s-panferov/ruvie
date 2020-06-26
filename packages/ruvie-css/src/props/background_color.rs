pub enum BackgroundColor {}
impl crate::Attribute for BackgroundColor {
    const NAME: &'static str = "background-color";
}
impl crate::StyleSheet {
    pub fn background_color<V: crate::ValueFor<BackgroundColor>>(mut self, value: V) -> Self {
        self.rules.insert("background-color", value.value());
        self
    }
}

impl crate::ValueFor<BackgroundColor> for crate::types::color::Color {}
