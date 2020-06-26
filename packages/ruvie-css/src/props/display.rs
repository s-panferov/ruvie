pub enum Display {}
impl crate::Attribute for Display {
    const NAME: &'static str = "display";
}
impl crate::StyleSheet {
    pub fn display<V: crate::ValueFor<Display>>(mut self, value: V) -> Self {
        self.rules.insert("display", value.value());
        self
    }
}
