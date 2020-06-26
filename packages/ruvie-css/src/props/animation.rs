pub enum Animation {}
impl crate::Attribute for Animation {
    const NAME: &'static str = "animation";
}
impl crate::StyleSheet {
    pub fn animation<V: crate::ValueFor<Animation>>(mut self, value: V) -> Self {
        self.rules.insert("animation", value.value());
        self
    }
}
