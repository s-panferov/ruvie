pub enum ClipPath {}
impl crate::Attribute for ClipPath {
    const NAME: &'static str = "clip-path";
}
impl crate::StyleSheet {
    pub fn clip_path<V: crate::ValueFor<ClipPath>>(mut self, value: V) -> Self {
        self.rules.insert("clip-path", value.value());
        self
    }
}
