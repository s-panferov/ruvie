pub enum BackfaceVisibility {
    Hidden,
    Visible,
}
impl std::fmt::Display for BackfaceVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackfaceVisibility::Hidden => write!(f, "hidden"),
            BackfaceVisibility::Visible => write!(f, "visible"),
        }
    }
}
impl crate::ValueFor<BackfaceVisibility> for BackfaceVisibility {}
impl crate::Attribute for BackfaceVisibility {
    const NAME: &'static str = "backface-visibility";
}
impl crate::StyleSheet {
    pub fn backface_visibility<V: crate::ValueFor<BackfaceVisibility>>(mut self, value: V) -> Self {
        self.rules.insert("backface-visibility", value.value());
        self
    }
}
