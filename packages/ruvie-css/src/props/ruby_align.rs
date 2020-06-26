pub enum RubyAlign {
    Center,
    SpaceAround,
    SpaceBetween,
    Start,
}
impl std::fmt::Display for RubyAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RubyAlign::Center => write!(f, "center"),
            RubyAlign::SpaceAround => write!(f, "space-around"),
            RubyAlign::SpaceBetween => write!(f, "space-between"),
            RubyAlign::Start => write!(f, "start"),
        }
    }
}
impl crate::ValueFor<RubyAlign> for RubyAlign {}
impl crate::Attribute for RubyAlign {
    const NAME: &'static str = "ruby-align";
}
impl crate::StyleSheet {
    pub fn ruby_align<V: crate::ValueFor<RubyAlign>>(mut self, value: V) -> Self {
        self.rules.insert("ruby-align", value.value());
        self
    }
}
