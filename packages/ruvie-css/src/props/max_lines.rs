pub enum MaxLine {
    None,
}
impl std::fmt::Display for MaxLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MaxLine::None => write!(f, "none"),
        }
    }
}
impl crate::ValueFor<MaxLine> for MaxLine {}
impl crate::Attribute for MaxLine {
    const NAME: &'static str = "max-lines";
}
impl crate::StyleSheet {
    pub fn max_lines<V: crate::ValueFor<MaxLine>>(mut self, value: V) -> Self {
        self.rules.insert("max-lines", value.value());
        self
    }
}

impl crate::ValueFor<MaxLine> for usize {}
impl crate::ValueFor<MaxLine> for isize {}
