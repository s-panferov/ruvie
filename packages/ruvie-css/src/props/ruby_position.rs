pub enum RubyPosition {
    InterCharacter,
    Over,
    Under,
}
impl std::fmt::Display for RubyPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RubyPosition::InterCharacter => write!(f, "inter-character"),
            RubyPosition::Over => write!(f, "over"),
            RubyPosition::Under => write!(f, "under"),
        }
    }
}
impl crate::ValueFor<RubyPosition> for RubyPosition {}
impl crate::Attribute for RubyPosition {
    const NAME: &'static str = "ruby-position";
}
impl crate::StyleSheet {
    pub fn ruby_position<V: crate::ValueFor<RubyPosition>>(mut self, value: V) -> Self {
        self.rules.insert("ruby-position", value.value());
        self
    }
}
