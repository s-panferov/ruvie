pub enum Perspective {
    None,
}
impl std::fmt::Display for Perspective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Perspective::None => write!(f, "none"),
        }
    }
}
impl crate::ValueFor<Perspective> for Perspective {}
impl crate::Attribute for Perspective {
    const NAME: &'static str = "perspective";
}
impl crate::StyleSheet {
    pub fn perspective<V: crate::ValueFor<Perspective>>(mut self, value: V) -> Self {
        self.rules.insert("perspective", value.value());
        self
    }
}

impl crate::ValueFor<Perspective> for crate::types::length::Length {}
