pub enum FontSynthesis {}
impl crate::Attribute for FontSynthesis {
    const NAME: &'static str = "font-synthesis";
}
impl crate::StyleSheet {
    pub fn font_synthesis<V: crate::ValueFor<FontSynthesis>>(mut self, value: V) -> Self {
        self.rules.insert("font-synthesis", value.value());
        self
    }
}
