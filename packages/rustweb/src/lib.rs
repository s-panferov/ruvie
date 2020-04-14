mod children;
mod component;
mod func;
mod instance;
mod layout;
mod scheduler;
mod target;
mod update;

pub mod dom;
pub mod prelude {
    pub use crate::func::FunctionalComponent;
    pub use crate::layout::LayoutBuilder;
}

pub use children::{Child, Children};
pub use component::{Component, Render};
pub use func::Func;
pub use instance::InstanceRef;
pub use layout::{AnyLayout, Layout, LayoutBuilder};
