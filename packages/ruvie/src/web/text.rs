use observe::{Const, Observable, Value};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::Node;

use super::{Web, WebContext};
use crate::element::TypedElement;
use crate::{
	component::Lifecycle,
	scope::Scope,
	target::{Html, Target},
	Children, Component, Element,
};

pub struct Text<T>
where
	T: Target<Realm = Html>,
{
	props: Value<String>,
	scope: Scope<Self, T>,
}

impl<T> Component<T> for Text<T>
where
	T: Target<Realm = Html>,
{
	type Props = Value<String>;

	fn create(props: Self::Props, scope: Scope<Self, T>) -> Self {
		Text { props, scope }
	}
}

impl Lifecycle<Web> for Text<Web> {
	fn mount(&mut self, ctx: &mut WebContext) -> Result<(), JsValue> {
		let el = ctx.doc.create_text_node("EMPTY");
		ctx.fragment.child(el.clone().unchecked_into::<Node>());
		ctx.reaction(self.scope.reaction(move |this, ctx| {
			el.set_data(&this.props.get(ctx.eval));
			Ok(())
		}));

		Ok(())
	}
}

impl<T: Target<Realm = Html>> From<String> for Element<T> {
	fn from(value: String) -> Self {
		Element::<T>::from(TypedElement::<Text<T>, T>::new(
			Const::new(value).into(),
			Children::from(None),
		))
	}
}

impl<T: Target<Realm = Html>> From<&str> for Element<T> {
	fn from(value: &str) -> Self {
		Element::<T>::from(TypedElement::<Text<T>, T>::new(
			Const::new(value.to_owned()).into(),
			Children::from(None),
		))
	}
}

impl<T: Target<Realm = Html>> From<Value<String>> for Children<T> {
	fn from(value: Value<String>) -> Self {
		Children::from(TypedElement::<Text<T>, T>::new(value, Children::from(None)))
	}
}

impl<T: Target<Realm = Html>> From<String> for Children<T> {
	fn from(value: String) -> Self {
		Children::<T>::from(Element::from(value))
	}
}

impl<T: Target<Realm = Html>> From<&str> for Children<T> {
	fn from(value: &str) -> Self {
		Children::<T>::from(Element::from(value))
	}
}
