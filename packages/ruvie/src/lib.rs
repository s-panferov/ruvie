#![feature(min_specialization)]

mod builder;
mod children;
mod component;
pub mod context;
pub mod contrib;
mod element;
mod error;
mod func;
mod handler;
mod html;
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

pub mod prelude {
	pub use crate::builder::*;
	pub use crate::component::{Component, ComponentExt};
	pub use crate::props::DynamicProps;
}

pub use children::Children;
pub use component::Component;
pub use element::Element;
pub use handler::Handler;
pub use props::Props;
pub use runtime::Runtime;
pub use scope::Scope;
pub use target::Target;
pub use view::View;
