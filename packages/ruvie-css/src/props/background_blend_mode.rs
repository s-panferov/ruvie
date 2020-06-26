pub enum BackgroundBlendMode {}
impl crate::Attribute for BackgroundBlendMode {
    const NAME: &'static str = "background-blend-mode";
}
impl crate::StyleSheet {
    pub fn background_blend_mode<V: crate::ValueFor<BackgroundBlendMode>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("background-blend-mode", value.value());
        self
    }
}
