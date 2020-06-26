pub enum BorderColor {}
impl crate::Attribute for BorderColor {
    const NAME: &'static str = "border-color";
}
impl crate::StyleSheet {
    pub fn border_color<V: crate::ValueFor<BorderColor>>(mut self, value: V) -> Self {
        self.rules.insert("border-color", value.value());
        self
    }
}
