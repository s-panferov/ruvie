pub enum PlaceSelf {}
impl crate::Attribute for PlaceSelf {
    const NAME: &'static str = "place-self";
}
impl crate::StyleSheet {
    pub fn place_self<V: crate::ValueFor<PlaceSelf>>(mut self, value: V) -> Self {
        self.rules.insert("place-self", value.value());
        self
    }
}
