pub enum FontSizeAdjust {
    None,
}
impl std::fmt::Display for FontSizeAdjust {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FontSizeAdjust::None => write!(f, "none"),
        }
    }
}
impl crate::ValueFor<FontSizeAdjust> for FontSizeAdjust {}
impl crate::Attribute for FontSizeAdjust {
    const NAME: &'static str = "font-size-adjust";
}
impl crate::StyleSheet {
    pub fn font_size_adjust<V: crate::ValueFor<FontSizeAdjust>>(mut self, value: V) -> Self {
        self.rules.insert("font-size-adjust", value.value());
        self
    }
}

impl crate::ValueFor<FontSizeAdjust> for usize {}
impl crate::ValueFor<FontSizeAdjust> for isize {}
impl crate::ValueFor<FontSizeAdjust> for f32 {}
impl crate::ValueFor<FontSizeAdjust> for f64 {}
