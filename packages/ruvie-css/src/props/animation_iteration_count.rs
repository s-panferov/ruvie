pub enum AnimationIterationCount {}
impl crate::Attribute for AnimationIterationCount {
    const NAME: &'static str = "animation-iteration-count";
}
impl crate::StyleSheet {
    pub fn animation_iteration_count<V: crate::ValueFor<AnimationIterationCount>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules
            .insert("animation-iteration-count", value.value());
        self
    }
}
