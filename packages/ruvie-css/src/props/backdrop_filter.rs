pub enum BackdropFilter {
    None,
}
impl std::fmt::Display for BackdropFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackdropFilter::None => write!(f, "none"),
        }
    }
}
impl crate::ValueFor<BackdropFilter> for BackdropFilter {}
impl crate::Attribute for BackdropFilter {
    const NAME: &'static str = "backdrop-filter";
}
impl crate::StyleSheet {
    pub fn backdrop_filter<V: crate::ValueFor<BackdropFilter>>(mut self, value: V) -> Self {
        self.rules.insert("backdrop-filter", value.value());
        self
    }
}
