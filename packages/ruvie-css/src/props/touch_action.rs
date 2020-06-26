pub enum TouchAction {}
impl crate::Attribute for TouchAction {
    const NAME: &'static str = "touch-action";
}
impl crate::StyleSheet {
    pub fn touch_action<V: crate::ValueFor<TouchAction>>(mut self, value: V) -> Self {
        self.rules.insert("touch-action", value.value());
        self
    }
}
