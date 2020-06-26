pub enum Widow {}
impl crate::Attribute for Widow {
    const NAME: &'static str = "widows";
}
impl crate::StyleSheet {
    pub fn widows<V: crate::ValueFor<Widow>>(mut self, value: V) -> Self {
        self.rules.insert("widows", value.value());
        self
    }
}

impl crate::ValueFor<Widow> for usize {}
impl crate::ValueFor<Widow> for isize {}
