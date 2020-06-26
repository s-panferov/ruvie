pub enum JustifySelf {}
impl crate::Attribute for JustifySelf {
    const NAME: &'static str = "justify-self";
}
impl crate::StyleSheet {
    pub fn justify_self<V: crate::ValueFor<JustifySelf>>(mut self, value: V) -> Self {
        self.rules.insert("justify-self", value.value());
        self
    }
}
