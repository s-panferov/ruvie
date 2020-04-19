use observe::{Local, Timed, Value, Var};
use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

type EventValue<T> = Value<Option<Timed<T>>, Local>;

#[derive(Clone)]
pub struct Event<T> {
    value: EventValue<T>,
}

impl<T> Debug for Event<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Event")
    }
}

impl<T> Default for Event<T>
where
    T: 'static,
{
    fn default() -> Self {
        Event::new()
    }
}

impl<T> Deref for Event<T>
where
    T: 'static,
{
    type Target = EventValue<T>;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Event<T>
where
    T: 'static,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T> Event<T>
where
    T: 'static,
{
    pub fn new() -> Event<T> {
        Event {
            value: Value::from(Var::new(None)),
        }
    }
}
