pub enum BoxLine {
    Multiple,
    Single,
}
impl std::fmt::Display for BoxLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoxLine::Multiple => write!(f, "multiple"),
            BoxLine::Single => write!(f, "single"),
        }
    }
}
impl crate::ValueFor<BoxLine> for BoxLine {}
impl crate::Attribute for BoxLine {
    const NAME: &'static str = "box-lines";
}
impl crate::StyleSheet {
    pub fn box_lines<V: crate::ValueFor<BoxLine>>(mut self, value: V) -> Self {
        self.rules.insert("box-lines", value.value());
        self
    }
}
