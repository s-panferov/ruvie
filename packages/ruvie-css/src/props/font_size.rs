pub enum FontSize {}
impl crate::Attribute for FontSize {
    const NAME: &'static str = "font-size";
}
impl crate::StyleSheet {
    pub fn font_size<V: crate::ValueFor<FontSize>>(mut self, value: V) -> Self {
        self.rules.insert("font-size", value.value());
        self
    }
}

impl crate::ValueFor<FontSize> for crate::types::length::Length {}
impl crate::ValueFor<FontSize> for crate::types::percentage::Percentage {}
