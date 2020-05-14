use std::{fmt::Display, hash::Hash};

#[derive(Debug)]
pub struct HashableError(anyhow::Error);

impl Display for HashableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<E> From<E> for HashableError
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn from(e: E) -> Self {
        HashableError(e.into())
    }
}

impl Hash for HashableError {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{:#?}", self.0).hash(state);
    }
}
