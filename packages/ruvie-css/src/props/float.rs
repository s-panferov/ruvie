pub enum Float {
    InlineEnd,
    InlineStart,
    Left,
    None,
    Right,
}
impl std::fmt::Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Float::InlineEnd => write!(f, "inline-end"),
            Float::InlineStart => write!(f, "inline-start"),
            Float::Left => write!(f, "left"),
            Float::None => write!(f, "none"),
            Float::Right => write!(f, "right"),
        }
    }
}
impl crate::ValueFor<Float> for Float {}
impl crate::Attribute for Float {
    const NAME: &'static str = "float";
}
impl crate::StyleSheet {
    pub fn float<V: crate::ValueFor<Float>>(mut self, value: V) -> Self {
        self.rules.insert("float", value.value());
        self
    }
}
