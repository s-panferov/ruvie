pub enum BorderRightColor {}
impl crate::Attribute for BorderRightColor {
    const NAME: &'static str = "border-right-color";
}
impl crate::StyleSheet {
    pub fn border_right_color<V: crate::ValueFor<BorderRightColor>>(mut self, value: V) -> Self {
        self.rules.insert("border-right-color", value.value());
        self
    }
}

impl crate::ValueFor<BorderRightColor> for crate::types::color::Color {}
