pub enum BoxOrient {
    BlockAxis,
    Horizontal,
    Inherit,
    InlineAxis,
    Vertical,
}
impl std::fmt::Display for BoxOrient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoxOrient::BlockAxis => write!(f, "block-axis"),
            BoxOrient::Horizontal => write!(f, "horizontal"),
            BoxOrient::Inherit => write!(f, "inherit"),
            BoxOrient::InlineAxis => write!(f, "inline-axis"),
            BoxOrient::Vertical => write!(f, "vertical"),
        }
    }
}
impl crate::ValueFor<BoxOrient> for BoxOrient {}
impl crate::Attribute for BoxOrient {
    const NAME: &'static str = "box-orient";
}
impl crate::StyleSheet {
    pub fn box_orient<V: crate::ValueFor<BoxOrient>>(mut self, value: V) -> Self {
        self.rules.insert("box-orient", value.value());
        self
    }
}
