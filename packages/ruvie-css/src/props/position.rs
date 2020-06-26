pub enum Position {
    Absolute,
    Fixed,
    Relative,
    Static,
    Sticky,
}
impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Position::Absolute => write!(f, "absolute"),
            Position::Fixed => write!(f, "fixed"),
            Position::Relative => write!(f, "relative"),
            Position::Static => write!(f, "static"),
            Position::Sticky => write!(f, "sticky"),
        }
    }
}
impl crate::ValueFor<Position> for Position {}
impl crate::Attribute for Position {
    const NAME: &'static str = "position";
}
impl crate::StyleSheet {
    pub fn position<V: crate::ValueFor<Position>>(mut self, value: V) -> Self {
        self.rules.insert("position", value.value());
        self
    }
}
