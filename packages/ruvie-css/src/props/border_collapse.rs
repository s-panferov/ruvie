pub enum BorderCollapse {
    Collapse,
    Separate,
}
impl std::fmt::Display for BorderCollapse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BorderCollapse::Collapse => write!(f, "collapse"),
            BorderCollapse::Separate => write!(f, "separate"),
        }
    }
}
impl crate::ValueFor<BorderCollapse> for BorderCollapse {}
impl crate::Attribute for BorderCollapse {
    const NAME: &'static str = "border-collapse";
}
impl crate::StyleSheet {
    pub fn border_collapse<V: crate::ValueFor<BorderCollapse>>(mut self, value: V) -> Self {
        self.rules.insert("border-collapse", value.value());
        self
    }
}
