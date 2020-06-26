pub enum TransformOrigin {}
impl crate::Attribute for TransformOrigin {
    const NAME: &'static str = "transform-origin";
}
impl crate::StyleSheet {
    pub fn transform_origin<V: crate::ValueFor<TransformOrigin>>(mut self, value: V) -> Self {
        self.rules.insert("transform-origin", value.value());
        self
    }
}
