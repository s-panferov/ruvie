pub enum LineHeight {
    Normal,
}
impl std::fmt::Display for LineHeight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineHeight::Normal => write!(f, "normal"),
        }
    }
}
impl crate::ValueFor<LineHeight> for LineHeight {}
impl crate::Attribute for LineHeight {
    const NAME: &'static str = "line-height";
}
impl crate::StyleSheet {
    pub fn line_height<V: crate::ValueFor<LineHeight>>(mut self, value: V) -> Self {
        self.rules.insert("line-height", value.value());
        self
    }
}

impl crate::ValueFor<LineHeight> for usize {}
impl crate::ValueFor<LineHeight> for isize {}
impl crate::ValueFor<LineHeight> for f32 {}
impl crate::ValueFor<LineHeight> for f64 {}

impl crate::ValueFor<LineHeight> for crate::types::length::Length {}

impl crate::ValueFor<LineHeight> for crate::types::percentage::Percentage {}
