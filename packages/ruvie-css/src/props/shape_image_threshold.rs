pub enum ShapeImageThreshold {}
impl crate::Attribute for ShapeImageThreshold {
    const NAME: &'static str = "shape-image-threshold";
}
impl crate::StyleSheet {
    pub fn shape_image_threshold<V: crate::ValueFor<ShapeImageThreshold>>(
        mut self,
        value: V,
    ) -> Self {
        self.rules.insert("shape-image-threshold", value.value());
        self
    }
}

impl crate::ValueFor<ShapeImageThreshold> for f32 {}
impl crate::ValueFor<ShapeImageThreshold> for f64 {}
