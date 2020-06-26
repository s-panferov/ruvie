pub enum PlaceContent {}
impl crate::Attribute for PlaceContent {
    const NAME: &'static str = "place-content";
}
impl crate::StyleSheet {
    pub fn place_content<V: crate::ValueFor<PlaceContent>>(mut self, value: V) -> Self {
        self.rules.insert("place-content", value.value());
        self
    }
}
