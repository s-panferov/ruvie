pub enum TextAlign {
    Center,
    End,
    Justify,
    Left,
    MatchParent,
    Right,
    Start,
}
impl std::fmt::Display for TextAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextAlign::Center => write!(f, "center"),
            TextAlign::End => write!(f, "end"),
            TextAlign::Justify => write!(f, "justify"),
            TextAlign::Left => write!(f, "left"),
            TextAlign::MatchParent => write!(f, "match-parent"),
            TextAlign::Right => write!(f, "right"),
            TextAlign::Start => write!(f, "start"),
        }
    }
}
impl crate::ValueFor<TextAlign> for TextAlign {}
impl crate::Attribute for TextAlign {
    const NAME: &'static str = "text-align";
}
impl crate::StyleSheet {
    pub fn text_align<V: crate::ValueFor<TextAlign>>(mut self, value: V) -> Self {
        self.rules.insert("text-align", value.value());
        self
    }
}
