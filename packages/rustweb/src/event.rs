use std::{fmt::Debug, rc::Rc};

pub struct Event<T> {
    handler: Rc<dyn Fn(T)>,
}

impl<T> Clone for Event<T> {
    fn clone(&self) -> Self {
        Event {
            handler: self.handler.clone(),
        }
    }
}

impl<T> Debug for Event<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Event")
    }
}

impl<T> Event<T> {
    pub fn new<F>(handler: F) -> Event<T>
    where
        F: Fn(T) + 'static,
    {
        Event {
            handler: Rc::new(handler),
        }
    }

    pub fn trigger(&self, ev: T) {
        (self.handler)(ev)
    }
}
