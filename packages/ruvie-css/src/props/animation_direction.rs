pub enum AnimationDirection {}
impl crate::Attribute for AnimationDirection {
    const NAME: &'static str = "animation-direction";
}
impl crate::StyleSheet {
    pub fn animation_direction<V: crate::ValueFor<AnimationDirection>>(mut self, value: V) -> Self {
        self.rules.insert("animation-direction", value.value());
        self
    }
}
