pub enum PerspectiveOrigin {}
impl crate::Attribute for PerspectiveOrigin {
    const NAME: &'static str = "perspective-origin";
}
impl crate::StyleSheet {
    pub fn perspective_origin<V: crate::ValueFor<PerspectiveOrigin>>(mut self, value: V) -> Self {
        self.rules.insert("perspective-origin", value.value());
        self
    }
}
