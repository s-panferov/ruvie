pub enum BorderSpacing {}
impl crate::Attribute for BorderSpacing {
    const NAME: &'static str = "border-spacing";
}
impl crate::StyleSheet {
    pub fn border_spacing<V: crate::ValueFor<BorderSpacing>>(mut self, value: V) -> Self {
        self.rules.insert("border-spacing", value.value());
        self
    }
}
