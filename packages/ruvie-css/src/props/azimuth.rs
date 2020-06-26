pub enum Azimuth {}
impl crate::Attribute for Azimuth {
    const NAME: &'static str = "azimuth";
}
impl crate::StyleSheet {
    pub fn azimuth<V: crate::ValueFor<Azimuth>>(mut self, value: V) -> Self {
        self.rules.insert("azimuth", value.value());
        self
    }
}
