pub enum BoxLines {
    Multiple,
    Single,
}
impl std::fmt::Display for BoxLines {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoxLines::Multiple => write!(f, "multiple"),
            BoxLines::Single => write!(f, "single"),
        }
    }
}
impl crate::ValueFor<BoxLines> for BoxLines {}
impl crate::Attribute for BoxLines {
    const NAME: &'static str = "box-lines";
}
impl crate::StyleSheet {
    pub fn box_lines<V: crate::ValueFor<BoxLines>>(mut self, value: V) -> Self {
        self.rules.insert("box-lines", value.value());
        self
    }
}
