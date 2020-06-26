pub enum PageBreakAfter {
    Always,
    Auto,
    Avoid,
    Left,
    Recto,
    Right,
    Verso,
}
impl std::fmt::Display for PageBreakAfter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PageBreakAfter::Always => write!(f, "always"),
            PageBreakAfter::Auto => write!(f, "auto"),
            PageBreakAfter::Avoid => write!(f, "avoid"),
            PageBreakAfter::Left => write!(f, "left"),
            PageBreakAfter::Recto => write!(f, "recto"),
            PageBreakAfter::Right => write!(f, "right"),
            PageBreakAfter::Verso => write!(f, "verso"),
        }
    }
}
impl crate::ValueFor<PageBreakAfter> for PageBreakAfter {}
impl crate::Attribute for PageBreakAfter {
    const NAME: &'static str = "page-break-after";
}
impl crate::StyleSheet {
    pub fn page_break_after<V: crate::ValueFor<PageBreakAfter>>(mut self, value: V) -> Self {
        self.rules.insert("page-break-after", value.value());
        self
    }
}
