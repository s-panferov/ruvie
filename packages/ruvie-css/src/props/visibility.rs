pub enum Visibility {
    Collapse,
    Hidden,
    Visible,
}
impl std::fmt::Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Visibility::Collapse => write!(f, "collapse"),
            Visibility::Hidden => write!(f, "hidden"),
            Visibility::Visible => write!(f, "visible"),
        }
    }
}
impl crate::ValueFor<Visibility> for Visibility {}
impl crate::Attribute for Visibility {
    const NAME: &'static str = "visibility";
}
impl crate::StyleSheet {
    pub fn visibility<V: crate::ValueFor<Visibility>>(mut self, value: V) -> Self {
        self.rules.insert("visibility", value.value());
        self
    }
}
