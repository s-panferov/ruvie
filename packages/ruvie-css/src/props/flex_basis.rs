pub enum FlexBasis {
    Content,
}
impl std::fmt::Display for FlexBasis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlexBasis::Content => write!(f, "content"),
        }
    }
}
impl crate::ValueFor<FlexBasis> for FlexBasis {}
impl crate::Attribute for FlexBasis {
    const NAME: &'static str = "flex-basis";
}
impl crate::StyleSheet {
    pub fn flex_basis<V: crate::ValueFor<FlexBasis>>(mut self, value: V) -> Self {
        self.rules.insert("flex-basis", value.value());
        self
    }
}
