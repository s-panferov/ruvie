pub enum Width {
    Auto,
    MaxContent,
    MinContent,
}
impl std::fmt::Display for Width {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Width::Auto => write!(f, "auto"),
            Width::MaxContent => write!(f, "max-content"),
            Width::MinContent => write!(f, "min-content"),
        }
    }
}
impl crate::ValueFor<Width> for Width {}
impl crate::Attribute for Width {
    const NAME: &'static str = "width";
}
impl crate::StyleSheet {
    pub fn width<V: crate::ValueFor<Width>>(mut self, value: V) -> Self {
        self.rules.insert("width", value.value());
        self
    }
}

impl crate::ValueFor<Width> for crate::types::length::Length {}

impl crate::ValueFor<Width> for crate::types::percentage::Percentage {}
