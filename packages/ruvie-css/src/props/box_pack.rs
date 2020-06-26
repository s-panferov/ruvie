pub enum BoxPack {
    Center,
    End,
    Justify,
    Start,
}
impl std::fmt::Display for BoxPack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoxPack::Center => write!(f, "center"),
            BoxPack::End => write!(f, "end"),
            BoxPack::Justify => write!(f, "justify"),
            BoxPack::Start => write!(f, "start"),
        }
    }
}
impl crate::ValueFor<BoxPack> for BoxPack {}
impl crate::Attribute for BoxPack {
    const NAME: &'static str = "box-pack";
}
impl crate::StyleSheet {
    pub fn box_pack<V: crate::ValueFor<BoxPack>>(mut self, value: V) -> Self {
        self.rules.insert("box-pack", value.value());
        self
    }
}
