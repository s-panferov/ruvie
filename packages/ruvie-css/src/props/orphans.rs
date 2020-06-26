pub enum Orphans {}
impl crate::Attribute for Orphans {
    const NAME: &'static str = "orphans";
}
impl crate::StyleSheet {
    pub fn orphans<V: crate::ValueFor<Orphans>>(mut self, value: V) -> Self {
        self.rules.insert("orphans", value.value());
        self
    }
}

impl crate::ValueFor<Orphans> for usize {}
impl crate::ValueFor<Orphans> for isize {}
