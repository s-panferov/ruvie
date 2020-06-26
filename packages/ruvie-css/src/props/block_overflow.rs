pub enum BlockOverflow {
    Clip,
    Ellipsi,
}
impl std::fmt::Display for BlockOverflow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockOverflow::Clip => write!(f, "clip"),
            BlockOverflow::Ellipsi => write!(f, "ellipsis"),
        }
    }
}
impl crate::ValueFor<BlockOverflow> for BlockOverflow {}
impl crate::Attribute for BlockOverflow {
    const NAME: &'static str = "block-overflow";
}
impl crate::StyleSheet {
    pub fn block_overflow<V: crate::ValueFor<BlockOverflow>>(mut self, value: V) -> Self {
        self.rules.insert("block-overflow", value.value());
        self
    }
}
