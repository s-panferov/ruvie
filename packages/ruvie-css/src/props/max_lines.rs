pub enum MaxLines {
    None,
}
impl std::fmt::Display for MaxLines {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MaxLines::None => write!(f, "none"),
        }
    }
}
impl crate::ValueFor<MaxLines> for MaxLines {}
impl crate::Attribute for MaxLines {
    const NAME: &'static str = "max-lines";
}
impl crate::StyleSheet {
    pub fn max_lines<V: crate::ValueFor<MaxLines>>(mut self, value: V) -> Self {
        self.rules.insert("max-lines", value.value());
        self
    }
}

impl crate::ValueFor<MaxLines> for usize {}
impl crate::ValueFor<MaxLines> for isize {}
