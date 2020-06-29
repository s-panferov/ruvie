use observe::{Const, Observable, Value};
use wasm_bindgen::JsCast;
use web_sys::Node;

use super::WebContext;
use crate::element::TypedElement;
use crate::{
	component::Constructor, context::Mount, error::RuvieError, scope::Scope, Children, Component,
	Element,
};
use std::any::Any;

pub struct Text {
	props: Value<String>,
	scope: Scope<Self>,
}

impl Constructor for Text {
	fn create(props: <Self as Component>::Props, scope: Scope<Self>) -> Self {
		Text { props, scope }
	}
}

impl Component for Text {
	type Props = Value<String>;

	fn mount(&mut self, ctx: &mut Mount, target: &mut dyn Any) -> Result<(), RuvieError> {
		if target.is::<WebContext>() {
			let target = target.downcast_mut::<WebContext>().unwrap();
			let el = target.doc.create_text_node("EMPTY");
			target.fragment.child(el.clone().unchecked_into::<Node>());
			ctx.reaction(self.scope.reaction(move |this, ctx| {
				el.set_data(&this.props.get(ctx.eval));
				Ok(())
			}));
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
