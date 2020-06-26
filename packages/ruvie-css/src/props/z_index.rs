pub enum ZIndex {
    Auto,
}
impl std::fmt::Display for ZIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ZIndex::Auto => write!(f, "auto"),
        }
    }
}
impl crate::ValueFor<ZIndex> for ZIndex {}
impl crate::Attribute for ZIndex {
    const NAME: &'static str = "z-index";
}
impl crate::StyleSheet {
    pub fn z_index<V: crate::ValueFor<ZIndex>>(mut self, value: V) -> Self {
        self.rules.insert("z-index", value.value());
        self
    }
}

impl crate::ValueFor<ZIndex> for usize {}
impl crate::ValueFor<ZIndex> for isize {}
