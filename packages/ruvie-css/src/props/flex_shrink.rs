pub enum FlexShrink {}
impl crate::Attribute for FlexShrink {
    const NAME: &'static str = "flex-shrink";
}
impl crate::StyleSheet {
    pub fn flex_shrink<V: crate::ValueFor<FlexShrink>>(mut self, value: V) -> Self {
        self.rules.insert("flex-shrink", value.value());
        self
    }
}

impl crate::ValueFor<FlexShrink> for usize {}
impl crate::ValueFor<FlexShrink> for isize {}
impl crate::ValueFor<FlexShrink> for f32 {}
impl crate::ValueFor<FlexShrink> for f64 {}
