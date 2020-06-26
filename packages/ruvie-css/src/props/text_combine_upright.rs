pub enum TextCombineUpright {}
impl crate::Attribute for TextCombineUpright {
    const NAME: &'static str = "text-combine-upright";
}
impl crate::StyleSheet {
    pub fn text_combine_upright<V: crate::ValueFor<TextCombineUpright>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("text-combine-upright", value.value());
        self
    }
}
