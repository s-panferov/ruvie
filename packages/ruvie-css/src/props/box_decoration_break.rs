pub enum BoxDecorationBreak {
    Clone,
    louse,
}
impl std::fmt::Display for BoxDecorationBreak {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoxDecorationBreak::Clone => write!(f, "clone"),
            BoxDecorationBreak::louse => write!(f, "slice"),
        }
    }
}
impl crate::ValueFor<BoxDecorationBreak> for BoxDecorationBreak {}
impl crate::Attribute for BoxDecorationBreak {
    const NAME: &'static str = "box-decoration-break";
}
impl crate::StyleSheet {
    pub fn box_decoration_break<V: crate::ValueFor<BoxDecorationBreak>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("box-decoration-break", value.value());
        self
    }
}
