pub enum UserSelect {
    All,
    Auto,
    Contain,
    None,
    Text,
}
impl std::fmt::Display for UserSelect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserSelect::All => write!(f, "all"),
            UserSelect::Auto => write!(f, "auto"),
            UserSelect::Contain => write!(f, "contain"),
            UserSelect::None => write!(f, "none"),
            UserSelect::Text => write!(f, "text"),
        }
    }
}
impl crate::ValueFor<UserSelect> for UserSelect {}
impl crate::Attribute for UserSelect {
    const NAME: &'static str = "user-select";
}
impl crate::StyleSheet {
    pub fn user_select<V: crate::ValueFor<UserSelect>>(mut self, value: V) -> Self {
        self.rules.insert("user-select", value.value());
        self
    }
}
