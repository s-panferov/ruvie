pub enum TabSize {}
impl crate::Attribute for TabSize {
    const NAME: &'static str = "tab-size";
}
impl crate::StyleSheet {
    pub fn tab_size<V: crate::ValueFor<TabSize>>(mut self, value: V) -> Self {
        self.rules.insert("tab-size", value.value());
        self
    }
}

impl crate::ValueFor<TabSize> for usize {}
impl crate::ValueFor<TabSize> for isize {}

impl crate::ValueFor<TabSize> for crate::types::length::Length {}
