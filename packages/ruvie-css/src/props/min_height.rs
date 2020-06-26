pub enum MinHeight {
    Auto,
    MaxContent,
    MinContent,
}
impl std::fmt::Display for MinHeight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MinHeight::Auto => write!(f, "auto"),
            MinHeight::MaxContent => write!(f, "max-content"),
            MinHeight::MinContent => write!(f, "min-content"),
        }
    }
}
impl crate::ValueFor<MinHeight> for MinHeight {}
impl crate::Attribute for MinHeight {
    const NAME: &'static str = "min-height";
}
impl crate::StyleSheet {
    pub fn min_height<V: crate::ValueFor<MinHeight>>(mut self, value: V) -> Self {
        self.rules.insert("min-height", value.value());
        self
    }
}

impl crate::ValueFor<MinHeight> for crate::types::length::Length {}

impl crate::ValueFor<MinHeight> for crate::types::percentage::Percentage {}
