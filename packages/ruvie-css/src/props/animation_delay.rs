pub enum AnimationDelay {}
impl crate::Attribute for AnimationDelay {
    const NAME: &'static str = "animation-delay";
}
impl crate::StyleSheet {
    pub fn animation_delay<V: crate::ValueFor<AnimationDelay>>(mut self, value: V) -> Self {
        self.rules.insert("animation-delay", value.value());
        self
    }
}
