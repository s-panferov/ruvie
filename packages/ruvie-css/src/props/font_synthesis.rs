pub enum Fontthesis {}
impl crate::Attribute for Fontthesis {
    const NAME: &'static str = "font-synthesis";
}
impl crate::StyleSheet {
    pub fn font_synthesis<V: crate::ValueFor<Fontthesis>>(mut self, value: V) -> Self {
        self.rules.insert("font-synthesis", value.value());
        self
    }
}
