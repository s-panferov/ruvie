pub enum FlexGrow {}
impl crate::Attribute for FlexGrow {
    const NAME: &'static str = "flex-grow";
}
impl crate::StyleSheet {
    pub fn flex_grow<V: crate::ValueFor<FlexGrow>>(mut self, value: V) -> Self {
        self.rules.insert("flex-grow", value.value());
        self
    }
}

impl crate::ValueFor<FlexGrow> for usize {}
impl crate::ValueFor<FlexGrow> for isize {}
impl crate::ValueFor<FlexGrow> for f32 {}
impl crate::ValueFor<FlexGrow> for f64 {}
