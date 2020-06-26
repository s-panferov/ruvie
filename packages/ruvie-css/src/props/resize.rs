pub enum Resize {
    Block,
    Both,
    Horizontal,
    Inline,
    None,
    Vertical,
}
impl std::fmt::Display for Resize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Resize::Block => write!(f, "block"),
            Resize::Both => write!(f, "both"),
            Resize::Horizontal => write!(f, "horizontal"),
            Resize::Inline => write!(f, "inline"),
            Resize::None => write!(f, "none"),
            Resize::Vertical => write!(f, "vertical"),
        }
    }
}
impl crate::ValueFor<Resize> for Resize {}
impl crate::Attribute for Resize {
    const NAME: &'static str = "resize";
}
impl crate::StyleSheet {
    pub fn resize<V: crate::ValueFor<Resize>>(mut self, value: V) -> Self {
        self.rules.insert("resize", value.value());
        self
    }
}
