pub enum Filter {
    None,
}
impl std::fmt::Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Filter::None => write!(f, "none"),
        }
    }
}
impl crate::ValueFor<Filter> for Filter {}
impl crate::Attribute for Filter {
    const NAME: &'static str = "filter";
}
impl crate::StyleSheet {
    pub fn filter<V: crate::ValueFor<Filter>>(mut self, value: V) -> Self {
        self.rules.insert("filter", value.value());
        self
    }
}
