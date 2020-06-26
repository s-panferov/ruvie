pub enum AnimationFillMode {}
impl crate::Attribute for AnimationFillMode {
    const NAME: &'static str = "animation-fill-mode";
}
impl crate::StyleSheet {
    pub fn animation_fill_mode<V: crate::ValueFor<AnimationFillMode>>(mut self, value: V) -> Self {
        self.rules.insert("animation-fill-mode", value.value());
        self
    }
}
