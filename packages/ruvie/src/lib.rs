mod builder;
mod children;
mod component;
pub mod context;
pub mod contrib;
mod element;
mod error;
mod func;
mod handler;
mod instance;
mod props;
mod reference;
mod runtime;
mod scope;
mod target;
mod test;
mod view;

#[cfg(feature = "css")]
pub use ruvie_css as css;

#[cfg(feature = "web")]
pub mod web;

#[cfg(feature = "ssr")]
pub mod ssr;

pub mod html;

pub mod prelude {
	pub use crate::builder::*;
	pub use crate::component::ComponentExt;
	pub use crate::func::{FunctionExt, FunctionWithPropsExt};
}

pub use children::Children;
pub use component::{Component, Constructor};
pub use element::Element;
pub use handler::Handler;
pub use props::Props;
pub use runtime::Runtime;
pub use scope::Scope;
pub use target::Target;
pub use view::View;
