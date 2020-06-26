pub enum InitialLetterAlign {
    Alphabetic,
    Auto,
    Hanging,
    Ideographic,
}
impl std::fmt::Display for InitialLetterAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InitialLetterAlign::Alphabetic => write!(f, "alphabetic"),
            InitialLetterAlign::Auto => write!(f, "auto"),
            InitialLetterAlign::Hanging => write!(f, "hanging"),
            InitialLetterAlign::Ideographic => write!(f, "ideographic"),
        }
    }
}
impl crate::ValueFor<InitialLetterAlign> for InitialLetterAlign {}
impl crate::Attribute for InitialLetterAlign {
    const NAME: &'static str = "initial-letter-align";
}
impl crate::StyleSheet {
    pub fn initial_letter_align<V: crate::ValueFor<InitialLetterAlign>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("initial-letter-align", value.value());
        self
    }
}
