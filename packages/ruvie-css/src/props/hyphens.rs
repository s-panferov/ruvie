pub enum Hyphens {
    Auto,
    Manual,
    None,
}
impl std::fmt::Display for Hyphens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Hyphens::Auto => write!(f, "auto"),
            Hyphens::Manual => write!(f, "manual"),
            Hyphens::None => write!(f, "none"),
        }
    }
}
impl crate::ValueFor<Hyphens> for Hyphens {}
impl crate::Attribute for Hyphens {
    const NAME: &'static str = "hyphens";
}
impl crate::StyleSheet {
    pub fn hyphens<V: crate::ValueFor<Hyphens>>(mut self, value: V) -> Self {
        self.rules.insert("hyphens", value.value());
        self
    }
}
