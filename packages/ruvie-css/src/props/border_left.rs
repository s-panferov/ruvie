pub enum BorderLeft {}
impl crate::Attribute for BorderLeft {
    const NAME: &'static str = "border-left";
}
impl crate::StyleSheet {
    pub fn border_left<V: crate::ValueFor<BorderLeft>>(mut self, value: V) -> Self {
        self.rules.insert("border-left", value.value());
        self
    }
}
