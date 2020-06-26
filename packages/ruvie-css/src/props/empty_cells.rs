pub enum EmptyCell {
    Hide,
    Show,
}
impl std::fmt::Display for EmptyCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmptyCell::Hide => write!(f, "hide"),
            EmptyCell::Show => write!(f, "show"),
        }
    }
}
impl crate::ValueFor<EmptyCell> for EmptyCell {}
impl crate::Attribute for EmptyCell {
    const NAME: &'static str = "empty-cells";
}
impl crate::StyleSheet {
    pub fn empty_cells<V: crate::ValueFor<EmptyCell>>(mut self, value: V) -> Self {
        self.rules.insert("empty-cells", value.value());
        self
    }
}
