pub enum Widows {}
impl crate::Attribute for Widows {
    const NAME: &'static str = "widows";
}
impl crate::StyleSheet {
    pub fn widows<V: crate::ValueFor<Widows>>(mut self, value: V) -> Self {
        self.rules.insert("widows", value.value());
        self
    }
}

impl crate::ValueFor<Widows> for usize {}
impl crate::ValueFor<Widows> for isize {}
