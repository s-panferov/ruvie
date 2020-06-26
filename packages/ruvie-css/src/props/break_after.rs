pub enum BreakAfter {
    All,
    Alway,
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
impl std::fmt::Display for BreakAfter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BreakAfter::All => write!(f, "all"),
            BreakAfter::Alway => write!(f, "always"),
            BreakAfter::Auto => write!(f, "auto"),
            BreakAfter::Avoid => write!(f, "avoid"),
            BreakAfter::AvoidColumn => write!(f, "avoid-column"),
            BreakAfter::AvoidPage => write!(f, "avoid-page"),
            BreakAfter::AvoidRegion => write!(f, "avoid-region"),
            BreakAfter::Column => write!(f, "column"),
            BreakAfter::Left => write!(f, "left"),
            BreakAfter::Page => write!(f, "page"),
            BreakAfter::Recto => write!(f, "recto"),
            BreakAfter::Region => write!(f, "region"),
            BreakAfter::Right => write!(f, "right"),
            BreakAfter::Verso => write!(f, "verso"),
        }
    }
}
impl crate::ValueFor<BreakAfter> for BreakAfter {}
impl crate::Attribute for BreakAfter {
    const NAME: &'static str = "break-after";
}
impl crate::StyleSheet {
    pub fn break_after<V: crate::ValueFor<BreakAfter>>(mut self, value: V) -> Self {
        self.rules.insert("break-after", value.value());
        self
    }
}
