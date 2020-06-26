pub enum ColorAdjust {
    Economy,
    Exact,
}
impl std::fmt::Display for ColorAdjust {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorAdjust::Economy => write!(f, "economy"),
            ColorAdjust::Exact => write!(f, "exact"),
        }
    }
}
impl crate::ValueFor<ColorAdjust> for ColorAdjust {}
impl crate::Attribute for ColorAdjust {
    const NAME: &'static str = "color-adjust";
}
impl crate::StyleSheet {
    pub fn color_adjust<V: crate::ValueFor<ColorAdjust>>(mut self, value: V) -> Self {
        self.rules.insert("color-adjust", value.value());
        self
    }
}
