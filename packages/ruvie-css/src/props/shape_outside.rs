pub enum ShapeOutside {}
impl crate::Attribute for ShapeOutside {
    const NAME: &'static str = "shape-outside";
}
impl crate::StyleSheet {
    pub fn shape_outside<V: crate::ValueFor<ShapeOutside>>(mut self, value: V) -> Self {
        self.rules.insert("shape-outside", value.value());
        self
    }
}
