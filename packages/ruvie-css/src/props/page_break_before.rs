pub enum PageBreakBefore {
    Alway,
    Auto,
    Avoid,
    Left,
    Recto,
    Right,
    Verso,
}
impl std::fmt::Display for PageBreakBefore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PageBreakBefore::Alway => write!(f, "always"),
            PageBreakBefore::Auto => write!(f, "auto"),
            PageBreakBefore::Avoid => write!(f, "avoid"),
            PageBreakBefore::Left => write!(f, "left"),
            PageBreakBefore::Recto => write!(f, "recto"),
            PageBreakBefore::Right => write!(f, "right"),
            PageBreakBefore::Verso => write!(f, "verso"),
        }
    }
}
impl crate::ValueFor<PageBreakBefore> for PageBreakBefore {}
impl crate::Attribute for PageBreakBefore {
    const NAME: &'static str = "page-break-before";
}
impl crate::StyleSheet {
    pub fn page_break_before<V: crate::ValueFor<PageBreakBefore>>(mut self, value: V) -> Self {
        self.rules.insert("page-break-before", value.value());
        self
    }
}
