pub enum PageBreakInside {
    Auto,
    Avoid,
}
impl std::fmt::Display for PageBreakInside {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PageBreakInside::Auto => write!(f, "auto"),
            PageBreakInside::Avoid => write!(f, "avoid"),
        }
    }
}
impl crate::ValueFor<PageBreakInside> for PageBreakInside {}
impl crate::Attribute for PageBreakInside {
    const NAME: &'static str = "page-break-inside";
}
impl crate::StyleSheet {
    pub fn page_break_inside<V: crate::ValueFor<PageBreakInside>>(mut self, value: V) -> Self {
        self.rules.insert("page-break-inside", value.value());
        self
    }
}
