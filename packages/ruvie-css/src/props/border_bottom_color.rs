pub enum BorderBottomColor {}
impl crate::Attribute for BorderBottomColor {
    const NAME: &'static str = "border-bottom-color";
}
impl crate::StyleSheet {
    pub fn border_bottom_color<V: crate::ValueFor<BorderBottomColor>>(mut self, value: V) -> Self {
        self.rules.insert("border-bottom-color", value.value());
        self
    }
}
