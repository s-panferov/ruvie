pub enum Orphan {}
impl crate::Attribute for Orphan {
    const NAME: &'static str = "orphans";
}
impl crate::StyleSheet {
    pub fn orphans<V: crate::ValueFor<Orphan>>(mut self, value: V) -> Self {
        self.rules.insert("orphans", value.value());
        self
    }
}

impl crate::ValueFor<Orphan> for usize {}
impl crate::ValueFor<Orphan> for isize {}
