pub enum ScrollbarColor {}
impl crate::Attribute for ScrollbarColor {
    const NAME: &'static str = "scrollbar-color";
}
impl crate::StyleSheet {
    pub fn scrollbar_color<V: crate::ValueFor<ScrollbarColor>>(mut self, value: V) -> Self {
        self.rules.insert("scrollbar-color", value.value());
        self
    }
}
