pub enum LineClamp {
    None,
}
impl std::fmt::Display for LineClamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineClamp::None => write!(f, "none"),
        }
    }
}
impl crate::ValueFor<LineClamp> for LineClamp {}
impl crate::Attribute for LineClamp {
    const NAME: &'static str = "line-clamp";
}
impl crate::StyleSheet {
    pub fn line_clamp<V: crate::ValueFor<LineClamp>>(mut self, value: V) -> Self {
        self.rules.insert("line-clamp", value.value());
        self
    }
}

impl crate::ValueFor<LineClamp> for usize {}
impl crate::ValueFor<LineClamp> for isize {}
