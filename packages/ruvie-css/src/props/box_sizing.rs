pub enum BoxSizing {
    BorderBox,
    ContentBox,
}
impl std::fmt::Display for BoxSizing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoxSizing::BorderBox => write!(f, "border-box"),
            BoxSizing::ContentBox => write!(f, "content-box"),
        }
    }
}
impl crate::ValueFor<BoxSizing> for BoxSizing {}
impl crate::Attribute for BoxSizing {
    const NAME: &'static str = "box-sizing";
}
impl crate::StyleSheet {
    pub fn box_sizing<V: crate::ValueFor<BoxSizing>>(mut self, value: V) -> Self {
        self.rules.insert("box-sizing", value.value());
        self
    }
}
