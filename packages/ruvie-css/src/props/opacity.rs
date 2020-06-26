pub enum Opacity {}
impl crate::Attribute for Opacity {
    const NAME: &'static str = "opacity";
}
impl crate::StyleSheet {
    pub fn opacity<V: crate::ValueFor<Opacity>>(mut self, value: V) -> Self {
        self.rules.insert("opacity", value.value());
        self
    }
}

impl crate::ValueFor<Opacity> for f32 {}
impl crate::ValueFor<Opacity> for f64 {}
