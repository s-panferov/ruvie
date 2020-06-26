pub enum ListStyleImage {
    None,
}
impl std::fmt::Display for ListStyleImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListStyleImage::None => write!(f, "none"),
        }
    }
}
impl crate::ValueFor<ListStyleImage> for ListStyleImage {}
impl crate::Attribute for ListStyleImage {
    const NAME: &'static str = "list-style-image";
}
impl crate::StyleSheet {
    pub fn list_style_image<V: crate::ValueFor<ListStyleImage>>(mut self, value: V) -> Self {
        self.rules.insert("list-style-image", value.value());
        self
    }
}
