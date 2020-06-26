pub enum FlexBasi {
    Content,
}
impl std::fmt::Display for FlexBasi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlexBasi::Content => write!(f, "content"),
        }
    }
}
impl crate::ValueFor<FlexBasi> for FlexBasi {}
impl crate::Attribute for FlexBasi {
    const NAME: &'static str = "flex-basis";
}
impl crate::StyleSheet {
    pub fn flex_basis<V: crate::ValueFor<FlexBasi>>(mut self, value: V) -> Self {
        self.rules.insert("flex-basis", value.value());
        self
    }
}
