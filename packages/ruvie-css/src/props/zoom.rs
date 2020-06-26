pub enum Zoom {
    Normal,
    Reset,
}
impl std::fmt::Display for Zoom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Zoom::Normal => write!(f, "normal"),
            Zoom::Reset => write!(f, "reset"),
        }
    }
}
impl crate::ValueFor<Zoom> for Zoom {}
impl crate::Attribute for Zoom {
    const NAME: &'static str = "zoom";
}
impl crate::StyleSheet {
    pub fn zoom<V: crate::ValueFor<Zoom>>(mut self, value: V) -> Self {
        self.rules.insert("zoom", value.value());
        self
    }
}

impl crate::ValueFor<Zoom> for usize {}
impl crate::ValueFor<Zoom> for isize {}
impl crate::ValueFor<Zoom> for f32 {}
impl crate::ValueFor<Zoom> for f64 {}

impl crate::ValueFor<Zoom> for crate::types::percentage::Percentage {}
