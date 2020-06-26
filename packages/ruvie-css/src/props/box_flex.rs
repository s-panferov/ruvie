pub enum BoxFlex {}
impl crate::Attribute for BoxFlex {
    const NAME: &'static str = "box-flex";
}
impl crate::StyleSheet {
    pub fn box_flex<V: crate::ValueFor<BoxFlex>>(mut self, value: V) -> Self {
        self.rules.insert("box-flex", value.value());
        self
    }
}

impl crate::ValueFor<BoxFlex> for usize {}
impl crate::ValueFor<BoxFlex> for isize {}
impl crate::ValueFor<BoxFlex> for f32 {}
impl crate::ValueFor<BoxFlex> for f64 {}
