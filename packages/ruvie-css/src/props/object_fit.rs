pub enum ObjectFit {
    Contain,
    Cover,
    Fill,
    None,
    ScaleDown,
}
impl std::fmt::Display for ObjectFit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectFit::Contain => write!(f, "contain"),
            ObjectFit::Cover => write!(f, "cover"),
            ObjectFit::Fill => write!(f, "fill"),
            ObjectFit::None => write!(f, "none"),
            ObjectFit::ScaleDown => write!(f, "scale-down"),
        }
    }
}
impl crate::ValueFor<ObjectFit> for ObjectFit {}
impl crate::Attribute for ObjectFit {
    const NAME: &'static str = "object-fit";
}
impl crate::StyleSheet {
    pub fn object_fit<V: crate::ValueFor<ObjectFit>>(mut self, value: V) -> Self {
        self.rules.insert("object-fit", value.value());
        self
    }
}
