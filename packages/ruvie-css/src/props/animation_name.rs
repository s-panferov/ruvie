pub enum AnimationName {}
impl crate::Attribute for AnimationName {
    const NAME: &'static str = "animation-name";
}
impl crate::StyleSheet {
    pub fn animation_name<V: crate::ValueFor<AnimationName>>(mut self, value: V) -> Self {
        self.rules.insert("animation-name", value.value());
        self
    }
}
