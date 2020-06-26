pub enum MarginTrim {
    All,
    InFlow,
    None,
}
impl std::fmt::Display for MarginTrim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MarginTrim::All => write!(f, "all"),
            MarginTrim::InFlow => write!(f, "in-flow"),
            MarginTrim::None => write!(f, "none"),
        }
    }
}
impl crate::ValueFor<MarginTrim> for MarginTrim {}
impl crate::Attribute for MarginTrim {
    const NAME: &'static str = "margin-trim";
}
impl crate::StyleSheet {
    pub fn margin_trim<V: crate::ValueFor<MarginTrim>>(mut self, value: V) -> Self {
        self.rules.insert("margin-trim", value.value());
        self
    }
}
