pub enum MaxHeight {
    Auto,
    MaxContent,
    MinContent,
}
impl std::fmt::Display for MaxHeight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MaxHeight::Auto => write!(f, "auto"),
            MaxHeight::MaxContent => write!(f, "max-content"),
            MaxHeight::MinContent => write!(f, "min-content"),
        }
    }
}
impl crate::ValueFor<MaxHeight> for MaxHeight {}
impl crate::Attribute for MaxHeight {
    const NAME: &'static str = "max-height";
}
impl crate::StyleSheet {
    pub fn max_height<V: crate::ValueFor<MaxHeight>>(mut self, value: V) -> Self {
        self.rules.insert("max-height", value.value());
        self
    }
}

impl crate::ValueFor<MaxHeight> for crate::types::length::Length {}

impl crate::ValueFor<MaxHeight> for crate::types::percentage::Percentage {}
