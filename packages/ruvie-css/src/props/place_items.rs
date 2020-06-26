pub enum PlaceItem {}
impl crate::Attribute for PlaceItem {
    const NAME: &'static str = "place-items";
}
impl crate::StyleSheet {
    pub fn place_items<V: crate::ValueFor<PlaceItem>>(mut self, value: V) -> Self {
        self.rules.insert("place-items", value.value());
        self
    }
}
