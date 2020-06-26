pub enum Quotes {}
impl crate::Attribute for Quotes {
    const NAME: &'static str = "quotes";
}
impl crate::StyleSheet {
    pub fn quotes<V: crate::ValueFor<Quotes>>(mut self, value: V) -> Self {
        self.rules.insert("quotes", value.value());
        self
    }
}
