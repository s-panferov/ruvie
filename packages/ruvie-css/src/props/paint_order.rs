pub enum PaintOrder {}
impl crate::Attribute for PaintOrder {
    const NAME: &'static str = "paint-order";
}
impl crate::StyleSheet {
    pub fn paint_order<V: crate::ValueFor<PaintOrder>>(mut self, value: V) -> Self {
        self.rules.insert("paint-order", value.value());
        self
    }
}
