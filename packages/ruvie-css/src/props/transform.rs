pub enum Transform {
    None,
}
impl std::fmt::Display for Transform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Transform::None => write!(f, "none"),
        }
    }
}
impl crate::ValueFor<Transform> for Transform {}
impl crate::Attribute for Transform {
    const NAME: &'static str = "transform";
}
impl crate::StyleSheet {
    pub fn transform<V: crate::ValueFor<Transform>>(mut self, value: V) -> Self {
        self.rules.insert("transform", value.value());
        self
    }
}
