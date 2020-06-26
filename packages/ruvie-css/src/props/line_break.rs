pub enum LineBreak {
    Anywhere,
    Auto,
    Loose,
    Normal,
    Strict,
}
impl std::fmt::Display for LineBreak {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBreak::Anywhere => write!(f, "anywhere"),
            LineBreak::Auto => write!(f, "auto"),
            LineBreak::Loose => write!(f, "loose"),
            LineBreak::Normal => write!(f, "normal"),
            LineBreak::Strict => write!(f, "strict"),
        }
    }
}
impl crate::ValueFor<LineBreak> for LineBreak {}
impl crate::Attribute for LineBreak {
    const NAME: &'static str = "line-break";
}
impl crate::StyleSheet {
    pub fn line_break<V: crate::ValueFor<LineBreak>>(mut self, value: V) -> Self {
        self.rules.insert("line-break", value.value());
        self
    }
}
