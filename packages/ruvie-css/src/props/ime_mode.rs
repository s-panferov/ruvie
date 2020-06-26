pub enum ImeMode {
    Active,
    Auto,
    Disabled,
    Inactive,
    Normal,
}
impl std::fmt::Display for ImeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImeMode::Active => write!(f, "active"),
            ImeMode::Auto => write!(f, "auto"),
            ImeMode::Disabled => write!(f, "disabled"),
            ImeMode::Inactive => write!(f, "inactive"),
            ImeMode::Normal => write!(f, "normal"),
        }
    }
}
impl crate::ValueFor<ImeMode> for ImeMode {}
impl crate::Attribute for ImeMode {
    const NAME: &'static str = "ime-mode";
}
impl crate::StyleSheet {
    pub fn ime_mode<V: crate::ValueFor<ImeMode>>(mut self, value: V) -> Self {
        self.rules.insert("ime-mode", value.value());
        self
    }
}
