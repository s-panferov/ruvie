pub enum ShapeMargin {}
impl crate::Attribute for ShapeMargin {
    const NAME: &'static str = "shape-margin";
}
impl crate::StyleSheet {
    pub fn shape_margin<V: crate::ValueFor<ShapeMargin>>(mut self, value: V) -> Self {
        self.rules.insert("shape-margin", value.value());
        self
    }
}

impl crate::ValueFor<ShapeMargin> for crate::types::length::Length {}
impl crate::ValueFor<ShapeMargin> for crate::types::percentage::Percentage {}
