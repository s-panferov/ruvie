pub enum InitialLetter {}
impl crate::Attribute for InitialLetter {
    const NAME: &'static str = "initial-letter";
}
impl crate::StyleSheet {
    pub fn initial_letter<V: crate::ValueFor<InitialLetter>>(mut self, value: V) -> Self {
        self.rules.insert("initial-letter", value.value());
        self
    }
}
