pub enum Order {}
impl crate::Attribute for Order {
    const NAME: &'static str = "order";
}
impl crate::StyleSheet {
    pub fn order<V: crate::ValueFor<Order>>(mut self, value: V) -> Self {
        self.rules.insert("order", value.value());
        self
    }
}

impl crate::ValueFor<Order> for usize {}
impl crate::ValueFor<Order> for isize {}
