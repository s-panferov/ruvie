pub enum RubyMerge {
    Auto,
    Collapse,
    Separate,
}
impl std::fmt::Display for RubyMerge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RubyMerge::Auto => write!(f, "auto"),
            RubyMerge::Collapse => write!(f, "collapse"),
            RubyMerge::Separate => write!(f, "separate"),
        }
    }
}
impl crate::ValueFor<RubyMerge> for RubyMerge {}
impl crate::Attribute for RubyMerge {
    const NAME: &'static str = "ruby-merge";
}
impl crate::StyleSheet {
    pub fn ruby_merge<V: crate::ValueFor<RubyMerge>>(mut self, value: V) -> Self {
        self.rules.insert("ruby-merge", value.value());
        self
    }
}
