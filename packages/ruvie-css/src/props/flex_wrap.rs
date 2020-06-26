pub enum FlexWrap {
    Nowrap,
    Wrap,
    WrapReverse,
}
impl std::fmt::Display for FlexWrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlexWrap::Nowrap => write!(f, "nowrap"),
            FlexWrap::Wrap => write!(f, "wrap"),
            FlexWrap::WrapReverse => write!(f, "wrap-reverse"),
        }
    }
}
impl crate::ValueFor<FlexWrap> for FlexWrap {}
impl crate::Attribute for FlexWrap {
    const NAME: &'static str = "flex-wrap";
}
impl crate::StyleSheet {
    pub fn flex_wrap<V: crate::ValueFor<FlexWrap>>(mut self, value: V) -> Self {
        self.rules.insert("flex-wrap", value.value());
        self
    }
}
