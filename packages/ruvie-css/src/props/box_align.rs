pub enum BoxAlign {
    Baseline,
    Center,
    End,
    Start,
    Stretch,
}
impl std::fmt::Display for BoxAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoxAlign::Baseline => write!(f, "baseline"),
            BoxAlign::Center => write!(f, "center"),
            BoxAlign::End => write!(f, "end"),
            BoxAlign::Start => write!(f, "start"),
            BoxAlign::Stretch => write!(f, "stretch"),
        }
    }
}
impl crate::ValueFor<BoxAlign> for BoxAlign {}
impl crate::Attribute for BoxAlign {
    const NAME: &'static str = "box-align";
}
impl crate::StyleSheet {
    pub fn box_align<V: crate::ValueFor<BoxAlign>>(mut self, value: V) -> Self {
        self.rules.insert("box-align", value.value());
        self
    }
}
