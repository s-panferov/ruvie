pub enum ObjectPosition {}
impl crate::Attribute for ObjectPosition {
    const NAME: &'static str = "object-position";
}
impl crate::StyleSheet {
    pub fn object_position<V: crate::ValueFor<ObjectPosition>>(mut self, value: V) -> Self {
        self.rules.insert("object-position", value.value());
        self
    }
}
