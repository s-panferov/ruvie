pub enum AnimationPlayState {}
impl crate::Attribute for AnimationPlayState {
    const NAME: &'static str = "animation-play-state";
}
impl crate::StyleSheet {
    pub fn animation_play_state<V: crate::ValueFor<AnimationPlayState>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("animation-play-state", value.value());
        self
    }
}
