pub enum MixBlendMode {}
impl crate::Attribute for MixBlendMode {
    const NAME: &'static str = "mix-blend-mode";
}
impl crate::StyleSheet {
    pub fn mix_blend_mode<V: crate::ValueFor<MixBlendMode>>(mut self, value: V) -> Self {
        self.rules.insert("mix-blend-mode", value.value());
        self
    }
}
