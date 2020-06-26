pub enum All {
    Inherit,
    Initial,
    Revert,
    Unset,
}
impl std::fmt::Display for All {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            All::Inherit => write!(f, "inherit"),
            All::Initial => write!(f, "initial"),
            All::Revert => write!(f, "revert"),
            All::Unset => write!(f, "unset"),
        }
    }
}
impl crate::ValueFor<All> for All {}
impl crate::Attribute for All {
    const NAME: &'static str = "all";
}
impl crate::StyleSheet {
    pub fn all<V: crate::ValueFor<All>>(mut self, value: V) -> Self {
        self.rules.insert("all", value.value());
        self
    }
}
