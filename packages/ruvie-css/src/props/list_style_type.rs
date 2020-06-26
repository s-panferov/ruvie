pub enum ListStyleType {
    None,
}
impl std::fmt::Display for ListStyleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListStyleType::None => write!(f, "none"),
        }
    }
}
impl crate::ValueFor<ListStyleType> for ListStyleType {}
impl crate::Attribute for ListStyleType {
    const NAME: &'static str = "list-style-type";
}
impl crate::StyleSheet {
    pub fn list_style_type<V: crate::ValueFor<ListStyleType>>(mut self, value: V) -> Self {
        self.rules.insert("list-style-type", value.value());
        self
    }
}
