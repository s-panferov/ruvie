pub enum FlexDirection {
    Column,
    ColumnReverse,
    Row,
    RowReverse,
}
impl std::fmt::Display for FlexDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlexDirection::Column => write!(f, "column"),
            FlexDirection::ColumnReverse => write!(f, "column-reverse"),
            FlexDirection::Row => write!(f, "row"),
            FlexDirection::RowReverse => write!(f, "row-reverse"),
        }
    }
}
impl crate::ValueFor<FlexDirection> for FlexDirection {}
impl crate::Attribute for FlexDirection {
    const NAME: &'static str = "flex-direction";
}
impl crate::StyleSheet {
    pub fn flex_direction<V: crate::ValueFor<FlexDirection>>(mut self, value: V) -> Self {
        self.rules.insert("flex-direction", value.value());
        self
    }
}
