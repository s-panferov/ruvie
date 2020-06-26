pub enum OffsetDistance {}
impl crate::Attribute for OffsetDistance {
    const NAME: &'static str = "offset-distance";
}
impl crate::StyleSheet {
    pub fn offset_distance<V: crate::ValueFor<OffsetDistance>>(mut self, value: V) -> Self {
        self.rules.insert("offset-distance", value.value());
        self
    }
}

impl crate::ValueFor<OffsetDistance> for crate::types::length::Length {}
impl crate::ValueFor<OffsetDistance> for crate::types::percentage::Percentage {}
