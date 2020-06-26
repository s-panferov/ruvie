pub enum AnimationDuration {}
impl crate::Attribute for AnimationDuration {
    const NAME: &'static str = "animation-duration";
}
impl crate::StyleSheet {
    pub fn animation_duration<V: crate::ValueFor<AnimationDuration>>(mut self, value: V) -> Self {
        self.rules.insert("animation-duration", value.value());
        self
    }
}
