pub enum TransformBox {
    BorderBox,
    ContentBox,
    FillBox,
    StrokeBox,
    ViewBox,
}
impl std::fmt::Display for TransformBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransformBox::BorderBox => write!(f, "border-box"),
            TransformBox::ContentBox => write!(f, "content-box"),
            TransformBox::FillBox => write!(f, "fill-box"),
            TransformBox::StrokeBox => write!(f, "stroke-box"),
            TransformBox::ViewBox => write!(f, "view-box"),
        }
    }
}
impl crate::ValueFor<TransformBox> for TransformBox {}
impl crate::Attribute for TransformBox {
    const NAME: &'static str = "transform-box";
}
impl crate::StyleSheet {
    pub fn transform_box<V: crate::ValueFor<TransformBox>>(mut self, value: V) -> Self {
        self.rules.insert("transform-box", value.value());
        self
    }
}
