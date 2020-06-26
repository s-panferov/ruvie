pub enum Appearance {
    Auto,
    Button,
    MenulistButton,
    None,
    Textfield,
}
impl std::fmt::Display for Appearance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Appearance::Auto => write!(f, "auto"),
            Appearance::Button => write!(f, "button"),
            Appearance::MenulistButton => write!(f, "menulist-button"),
            Appearance::None => write!(f, "none"),
            Appearance::Textfield => write!(f, "textfield"),
        }
    }
}
impl crate::ValueFor<Appearance> for Appearance {}
impl crate::Attribute for Appearance {
    const NAME: &'static str = "appearance";
}
impl crate::StyleSheet {
    pub fn appearance<V: crate::ValueFor<Appearance>>(mut self, value: V) -> Self {
        self.rules.insert("appearance", value.value());
        self
    }
}
