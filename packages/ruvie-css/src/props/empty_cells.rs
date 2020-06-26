pub enum EmptyCells {
    Hide,
    Show,
}
impl std::fmt::Display for EmptyCells {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmptyCells::Hide => write!(f, "hide"),
            EmptyCells::Show => write!(f, "show"),
        }
    }
}
impl crate::ValueFor<EmptyCells> for EmptyCells {}
impl crate::Attribute for EmptyCells {
    const NAME: &'static str = "empty-cells";
}
impl crate::StyleSheet {
    pub fn empty_cells<V: crate::ValueFor<EmptyCells>>(mut self, value: V) -> Self {
        self.rules.insert("empty-cells", value.value());
        self
    }
}
