pub enum AlignSelf {}
impl crate::Attribute for AlignSelf {
    const NAME: &'static str = "align-self";
}
impl crate::StyleSheet {
    pub fn align_self<V: crate::ValueFor<AlignSelf>>(mut self, value: V) -> Self {
        self.rules.insert("align-self", value.value());
        self
    }
}
