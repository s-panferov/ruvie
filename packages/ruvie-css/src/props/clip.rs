pub enum Clip {
    Auto,
}
impl std::fmt::Display for Clip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Clip::Auto => write!(f, "auto"),
        }
    }
}
impl crate::ValueFor<Clip> for Clip {}
impl crate::Attribute for Clip {
    const NAME: &'static str = "clip";
}
impl crate::StyleSheet {
    pub fn clip<V: crate::ValueFor<Clip>>(mut self, value: V) -> Self {
        self.rules.insert("clip", value.value());
        self
    }
}
