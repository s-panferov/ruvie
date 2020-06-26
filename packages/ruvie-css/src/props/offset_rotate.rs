pub enum OffsetRotate {}
impl crate::Attribute for OffsetRotate {
    const NAME: &'static str = "offset-rotate";
}
impl crate::StyleSheet {
    pub fn offset_rotate<V: crate::ValueFor<OffsetRotate>>(mut self, value: V) -> Self {
        self.rules.insert("offset-rotate", value.value());
        self
    }
}
