pub enum Hyphen {
    Auto,
    Manual,
    None,
}
impl std::fmt::Display for Hyphen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Hyphen::Auto => write!(f, "auto"),
            Hyphen::Manual => write!(f, "manual"),
            Hyphen::None => write!(f, "none"),
        }
    }
}
impl crate::ValueFor<Hyphen> for Hyphen {}
impl crate::Attribute for Hyphen {
    const NAME: &'static str = "hyphens";
}
impl crate::StyleSheet {
    pub fn hyphens<V: crate::ValueFor<Hyphen>>(mut self, value: V) -> Self {
        self.rules.insert("hyphens", value.value());
        self
    }
}
