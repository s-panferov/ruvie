pub enum BorderTop {}
impl crate::Attribute for BorderTop {
    const NAME: &'static str = "border-top";
}
impl crate::StyleSheet {
    pub fn border_top<V: crate::ValueFor<BorderTop>>(mut self, value: V) -> Self {
        self.rules.insert("border-top", value.value());
        self
    }
}
