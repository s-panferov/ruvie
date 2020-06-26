pub enum BreakInside {
    Auto,
    Avoid,
    AvoidColumn,
    AvoidPage,
    AvoidRegion,
}
impl std::fmt::Display for BreakInside {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BreakInside::Auto => write!(f, "auto"),
            BreakInside::Avoid => write!(f, "avoid"),
            BreakInside::AvoidColumn => write!(f, "avoid-column"),
            BreakInside::AvoidPage => write!(f, "avoid-page"),
            BreakInside::AvoidRegion => write!(f, "avoid-region"),
        }
    }
}
impl crate::ValueFor<BreakInside> for BreakInside {}
impl crate::Attribute for BreakInside {
    const NAME: &'static str = "break-inside";
}
impl crate::StyleSheet {
    pub fn break_inside<V: crate::ValueFor<BreakInside>>(mut self, value: V) -> Self {
        self.rules.insert("break-inside", value.value());
        self
    }
}
