pub enum PlaceItems {}
impl crate::Attribute for PlaceItems {
    const NAME: &'static str = "place-items";
}
impl crate::StyleSheet {
    pub fn place_items<V: crate::ValueFor<PlaceItems>>(mut self, value: V) -> Self {
        self.rules.insert("place-items", value.value());
        self
    }
}
