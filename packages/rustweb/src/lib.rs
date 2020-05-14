mod builder;
mod children;
mod component;
pub mod context;
pub mod contrib;
mod event;
mod func;
mod instance;
mod layout;
mod props;
mod reference;
mod render;
mod runtime;
mod target;

#[cfg(feature = "dom")]
pub mod dom;

pub mod prelude {
    pub use crate::builder::*;
    pub use crate::component::{Component, ComponentExt};
    pub use crate::func::FunctionalComponent;
}

pub use children::Children;
pub use component::Component;
pub use event::Event;
pub use func::Func;
pub use instance::Instance;
pub use layout::{Child, Layout};
pub use props::Props;
pub use reference::ComponentRef;
pub use runtime::Runtime;
