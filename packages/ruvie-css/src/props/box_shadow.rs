pub enum BoxShadow {}
impl crate::Attribute for BoxShadow {
    const NAME: &'static str = "box-shadow";
}
impl crate::StyleSheet {
    pub fn box_shadow<V: crate::ValueFor<BoxShadow>>(mut self, value: V) -> Self {
        self.rules.insert("box-shadow", value.value());
        self
    }
}
