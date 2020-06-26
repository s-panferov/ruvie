pub enum UnicodeBidi {
    BidiOverride,
    Embed,
    Isolate,
    IsolateOverride,
    Normal,
    Plaintext,
}
impl std::fmt::Display for UnicodeBidi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnicodeBidi::BidiOverride => write!(f, "bidi-override"),
            UnicodeBidi::Embed => write!(f, "embed"),
            UnicodeBidi::Isolate => write!(f, "isolate"),
            UnicodeBidi::IsolateOverride => write!(f, "isolate-override"),
            UnicodeBidi::Normal => write!(f, "normal"),
            UnicodeBidi::Plaintext => write!(f, "plaintext"),
        }
    }
}
impl crate::ValueFor<UnicodeBidi> for UnicodeBidi {}
impl crate::Attribute for UnicodeBidi {
    const NAME: &'static str = "unicode-bidi";
}
impl crate::StyleSheet {
    pub fn unicode_bidi<V: crate::ValueFor<UnicodeBidi>>(mut self, value: V) -> Self {
        self.rules.insert("unicode-bidi", value.value());
        self
    }
}
