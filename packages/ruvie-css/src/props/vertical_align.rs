pub enum VerticalAlign {
    Baseline,
    Bottom,
    Middle,
    Sub,
    Super,
    TextBottom,
    TextTop,
    Top,
}
impl std::fmt::Display for VerticalAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerticalAlign::Baseline => write!(f, "baseline"),
            VerticalAlign::Bottom => write!(f, "bottom"),
            VerticalAlign::Middle => write!(f, "middle"),
            VerticalAlign::Sub => write!(f, "sub"),
            VerticalAlign::Super => write!(f, "super"),
            VerticalAlign::TextBottom => write!(f, "text-bottom"),
            VerticalAlign::TextTop => write!(f, "text-top"),
            VerticalAlign::Top => write!(f, "top"),
        }
    }
}
impl crate::ValueFor<VerticalAlign> for VerticalAlign {}
impl crate::Attribute for VerticalAlign {
    const NAME: &'static str = "vertical-align";
}
impl crate::StyleSheet {
    pub fn vertical_align<V: crate::ValueFor<VerticalAlign>>(mut self, value: V) -> Self {
        self.rules.insert("vertical-align", value.value());
        self
    }
}

impl crate::ValueFor<VerticalAlign> for crate::types::percentage::Percentage {}

impl crate::ValueFor<VerticalAlign> for crate::types::length::Length {}
