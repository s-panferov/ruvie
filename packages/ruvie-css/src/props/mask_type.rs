pub enum MaskType {
    Alpha,
    Luminance,
}
impl std::fmt::Display for MaskType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MaskType::Alpha => write!(f, "alpha"),
            MaskType::Luminance => write!(f, "luminance"),
        }
    }
}
impl crate::ValueFor<MaskType> for MaskType {}
impl crate::Attribute for MaskType {
    const NAME: &'static str = "mask-type";
}
impl crate::StyleSheet {
    pub fn mask_type<V: crate::ValueFor<MaskType>>(mut self, value: V) -> Self {
        self.rules.insert("mask-type", value.value());
        self
    }
}
