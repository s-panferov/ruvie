use observe::{Const, Value};

use crate::element::TypedElement;
use crate::{
	component::Constructor, context::Mount, error::RuvieError, scope::Scope, Children, Component,
	Element,
};

use std::any::Any;

pub struct Text {
	pub(crate) props: Value<String>,
}

impl Constructor for Text {
	fn create(props: <Self as Component>::Props, _: Scope<Self>) -> Self {
		Text { props }
	}
}

impl Component for Text {
	type Props = Value<String>;

	fn mount(&mut self, ctx: &mut Mount, impl_ctx: &mut dyn Any) -> Result<(), RuvieError> {
		#[cfg(feature = "web")]
		if impl_ctx.is::<crate::web::WebContext>() {
			let impl_ctx = impl_ctx.downcast_mut::<crate::web::WebContext>().unwrap();
			self.mount_web(ctx, impl_ctx)?
		}

		#[cfg(feature = "ssr")]
		if impl_ctx.is::<crate::ssr::Static>() {
			let impl_ctx = impl_ctx
				.downcast_mut::<crate::ssr::StaticContext>()
				.unwrap();
			self.mount_static(ctx, impl_ctx)?
		}

		Ok(())
	}
}

impl From<String> for Element {
	fn from(value: String) -> Self {
		Element::from(TypedElement::<Text>::new(
			Const::new(value).into(),
			Children::from(None),
		))
	}
}

impl From<&str> for Element {
	fn from(value: &str) -> Self {
		Element::from(TypedElement::<Text>::new(
			Const::new(value.to_owned()).into(),
			Children::from(None),
		))
	}
}

impl From<Value<String>> for Children {
	fn from(value: Value<String>) -> Self {
		Children::from(TypedElement::<Text>::new(value, Children::from(None)))
	}
}

impl From<String> for Children {
	fn from(value: String) -> Self {
		Children::from(Element::from(value))
	}
}

impl From<&str> for Children {
	fn from(value: &str) -> Self {
		Children::from(Element::from(value))
	}
}
