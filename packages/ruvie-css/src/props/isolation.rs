pub enum Isolation {
    Auto,
    Isolate,
}
impl std::fmt::Display for Isolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Isolation::Auto => write!(f, "auto"),
            Isolation::Isolate => write!(f, "isolate"),
        }
    }
}
impl crate::ValueFor<Isolation> for Isolation {}
impl crate::Attribute for Isolation {
    const NAME: &'static str = "isolation";
}
impl crate::StyleSheet {
    pub fn isolation<V: crate::ValueFor<Isolation>>(mut self, value: V) -> Self {
        self.rules.insert("isolation", value.value());
        self
    }
}
