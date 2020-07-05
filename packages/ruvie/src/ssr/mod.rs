mod element;
mod stylesheet;
mod target;
mod text;
mod utils;

#[cfg(test)]
mod test;

pub use target::{Static, StaticContext, StaticElementState};
pub use utils::stringify;
