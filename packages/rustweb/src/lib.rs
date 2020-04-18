mod after;
mod children;
mod component;
mod event;
mod func;
mod instance;
mod layout;
mod mount;
mod reference;
mod runtime;
mod target;
mod update;

pub mod dom;
pub mod prelude {
    pub use crate::component::Component;
    pub use crate::func::FunctionalComponent;
}

pub use after::AfterRender;
pub use children::{Child, Children};
pub use component::{Component, Render, RenderSelf};
pub use func::Func;
pub use instance::Instance;
pub use layout::{AnyLayout, Layout};
pub use reference::ComponentRef;
