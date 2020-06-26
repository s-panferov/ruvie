pub enum AnimationTimingFunction {}
impl crate::Attribute for AnimationTimingFunction {
    const NAME: &'static str = "animation-timing-function";
}
impl crate::StyleSheet {
    pub fn animation_timing_function<V: crate::ValueFor<AnimationTimingFunction>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules
            .insert("animation-timing-function", value.value());
        self
    }
}
