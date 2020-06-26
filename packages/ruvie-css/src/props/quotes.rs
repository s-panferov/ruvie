pub enum Quote {}
impl crate::Attribute for Quote {
    const NAME: &'static str = "quotes";
}
impl crate::StyleSheet {
    pub fn quotes<V: crate::ValueFor<Quote>>(mut self, value: V) -> Self {
        self.rules.insert("quotes", value.value());
        self
    }
}
