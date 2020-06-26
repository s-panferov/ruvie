pub enum BoxFlexGroup {}
impl crate::Attribute for BoxFlexGroup {
    const NAME: &'static str = "box-flex-group";
}
impl crate::StyleSheet {
    pub fn box_flex_group<V: crate::ValueFor<BoxFlexGroup>>(mut self, value: V) -> Self {
        self.rules.insert("box-flex-group", value.value());
        self
    }
}

impl crate::ValueFor<BoxFlexGroup> for usize {}
impl crate::ValueFor<BoxFlexGroup> for isize {}
