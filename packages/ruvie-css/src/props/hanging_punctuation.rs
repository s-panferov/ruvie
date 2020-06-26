pub enum HangingPunctuation {}
impl crate::Attribute for HangingPunctuation {
    const NAME: &'static str = "hanging-punctuation";
}
impl crate::StyleSheet {
    pub fn hanging_punctuation<V: crate::ValueFor<HangingPunctuation>>(mut self, value: V) -> Self {
        self.rules.insert("hanging-punctuation", value.value());
        self
    }
}
