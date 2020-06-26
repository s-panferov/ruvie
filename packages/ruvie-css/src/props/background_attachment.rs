pub enum BackgroundAttachment {}
impl crate::Attribute for BackgroundAttachment {
    const NAME: &'static str = "background-attachment";
}
impl crate::StyleSheet {
    pub fn background_attachment<V: crate::ValueFor<BackgroundAttachment>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("background-attachment", value.value());
        self
    }
}
