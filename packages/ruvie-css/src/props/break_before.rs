pub enum BreakBefore {
    All,
    Always,
    Auto,
    Avoid,
    AvoidColumn,
    AvoidPage,
    AvoidRegion,
    Column,
    Left,
    Page,
    Recto,
    Region,
    Right,
    Verso,
}
impl std::fmt::Display for BreakBefore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BreakBefore::All => write!(f, "all"),
            BreakBefore::Always => write!(f, "always"),
            BreakBefore::Auto => write!(f, "auto"),
            BreakBefore::Avoid => write!(f, "avoid"),
            BreakBefore::AvoidColumn => write!(f, "avoid-column"),
            BreakBefore::AvoidPage => write!(f, "avoid-page"),
            BreakBefore::AvoidRegion => write!(f, "avoid-region"),
            BreakBefore::Column => write!(f, "column"),
            BreakBefore::Left => write!(f, "left"),
            BreakBefore::Page => write!(f, "page"),
            BreakBefore::Recto => write!(f, "recto"),
            BreakBefore::Region => write!(f, "region"),
            BreakBefore::Right => write!(f, "right"),
            BreakBefore::Verso => write!(f, "verso"),
        }
    }
}
impl crate::ValueFor<BreakBefore> for BreakBefore {}
impl crate::Attribute for BreakBefore {
    const NAME: &'static str = "break-before";
}
impl crate::StyleSheet {
    pub fn break_before<V: crate::ValueFor<BreakBefore>>(mut self, value: V) -> Self {
        self.rules.insert("break-before", value.value());
        self
    }
}
