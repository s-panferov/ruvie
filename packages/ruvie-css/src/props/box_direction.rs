pub enum BoxDirection {
    Inherit,
    Normal,
    Reverse,
}
impl std::fmt::Display for BoxDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoxDirection::Inherit => write!(f, "inherit"),
            BoxDirection::Normal => write!(f, "normal"),
            BoxDirection::Reverse => write!(f, "reverse"),
        }
    }
}
impl crate::ValueFor<BoxDirection> for BoxDirection {}
impl crate::Attribute for BoxDirection {
    const NAME: &'static str = "box-direction";
}
impl crate::StyleSheet {
    pub fn box_direction<V: crate::ValueFor<BoxDirection>>(mut self, value: V) -> Self {
        self.rules.insert("box-direction", value.value());
        self
    }
}
