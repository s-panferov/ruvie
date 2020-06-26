pub enum Clear {
    Both,
    InlineEnd,
    InlineStart,
    Left,
    None,
    Right,
}
impl std::fmt::Display for Clear {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Clear::Both => write!(f, "both"),
            Clear::InlineEnd => write!(f, "inline-end"),
            Clear::InlineStart => write!(f, "inline-start"),
            Clear::Left => write!(f, "left"),
            Clear::None => write!(f, "none"),
            Clear::Right => write!(f, "right"),
        }
    }
}
impl crate::ValueFor<Clear> for Clear {}
impl crate::Attribute for Clear {
    const NAME: &'static str = "clear";
}
impl crate::StyleSheet {
    pub fn clear<V: crate::ValueFor<Clear>>(mut self, value: V) -> Self {
        self.rules.insert("clear", value.value());
        self
    }
}
