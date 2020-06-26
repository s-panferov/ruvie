pub enum TextAlignLast {
    Auto,
    Center,
    End,
    Justify,
    Left,
    Right,
    Start,
}
impl std::fmt::Display for TextAlignLast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextAlignLast::Auto => write!(f, "auto"),
            TextAlignLast::Center => write!(f, "center"),
            TextAlignLast::End => write!(f, "end"),
            TextAlignLast::Justify => write!(f, "justify"),
            TextAlignLast::Left => write!(f, "left"),
            TextAlignLast::Right => write!(f, "right"),
            TextAlignLast::Start => write!(f, "start"),
        }
    }
}
impl crate::ValueFor<TextAlignLast> for TextAlignLast {}
impl crate::Attribute for TextAlignLast {
    const NAME: &'static str = "text-align-last";
}
impl crate::StyleSheet {
    pub fn text_align_last<V: crate::ValueFor<TextAlignLast>>(mut self, value: V) -> Self {
        self.rules.insert("text-align-last", value.value());
        self
    }
}
