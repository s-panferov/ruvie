mod component;
mod instance;
mod layout;
mod scheduler;

pub mod dom;
pub mod prelude {
    pub use crate::component::FunctionalComponent;
    pub use crate::layout::LayoutBuilder;
}

pub use component::{Component, Context, Func};
pub use instance::InstanceRef;
pub use layout::{AnyLayout, Child, Children, Layout, LayoutBuilder};
