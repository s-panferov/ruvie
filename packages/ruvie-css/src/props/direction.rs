pub enum Direction {
    Ltr,
    Rtl,
}
impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Ltr => write!(f, "ltr"),
            Direction::Rtl => write!(f, "rtl"),
        }
    }
}
impl crate::ValueFor<Direction> for Direction {}
impl crate::Attribute for Direction {
    const NAME: &'static str = "direction";
}
impl crate::StyleSheet {
    pub fn direction<V: crate::ValueFor<Direction>>(mut self, value: V) -> Self {
        self.rules.insert("direction", value.value());
        self
    }
}
