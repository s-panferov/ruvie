pub enum ImageRendering {
    Auto,
    CrispEdge,
    Pixelated,
}
impl std::fmt::Display for ImageRendering {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageRendering::Auto => write!(f, "auto"),
            ImageRendering::CrispEdge => write!(f, "crisp-edges"),
            ImageRendering::Pixelated => write!(f, "pixelated"),
        }
    }
}
impl crate::ValueFor<ImageRendering> for ImageRendering {}
impl crate::Attribute for ImageRendering {
    const NAME: &'static str = "image-rendering";
}
impl crate::StyleSheet {
    pub fn image_rendering<V: crate::ValueFor<ImageRendering>>(mut self, value: V) -> Self {
        self.rules.insert("image-rendering", value.value());
        self
    }
}
