pub enum Flex {}
impl crate::Attribute for Flex {
    const NAME: &'static str = "flex";
}
impl crate::StyleSheet {
    pub fn flex<V: crate::ValueFor<Flex>>(mut self, value: V) -> Self {
        self.rules.insert("flex", value.value());
        self
    }
}
