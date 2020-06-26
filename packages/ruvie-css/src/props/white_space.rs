pub enum WhiteSpace {
    BreakSpaces,
    Normal,
    Nowrap,
    Pre,
    PreLine,
    PreWrap,
}
impl std::fmt::Display for WhiteSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WhiteSpace::BreakSpaces => write!(f, "break-spaces"),
            WhiteSpace::Normal => write!(f, "normal"),
            WhiteSpace::Nowrap => write!(f, "nowrap"),
            WhiteSpace::Pre => write!(f, "pre"),
            WhiteSpace::PreLine => write!(f, "pre-line"),
            WhiteSpace::PreWrap => write!(f, "pre-wrap"),
        }
    }
}
impl crate::ValueFor<WhiteSpace> for WhiteSpace {}
impl crate::Attribute for WhiteSpace {
    const NAME: &'static str = "white-space";
}
impl crate::StyleSheet {
    pub fn white_space<V: crate::ValueFor<WhiteSpace>>(mut self, value: V) -> Self {
        self.rules.insert("white-space", value.value());
        self
    }
}
