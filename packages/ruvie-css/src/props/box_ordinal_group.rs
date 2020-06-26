pub enum BoxOrdinalGroup {}
impl crate::Attribute for BoxOrdinalGroup {
    const NAME: &'static str = "box-ordinal-group";
}
impl crate::StyleSheet {
    pub fn box_ordinal_group<V: crate::ValueFor<BoxOrdinalGroup>>(mut self, value: V) -> Self {
        self.rules.insert("box-ordinal-group", value.value());
        self
    }
}

impl crate::ValueFor<BoxOrdinalGroup> for usize {}
impl crate::ValueFor<BoxOrdinalGroup> for isize {}
